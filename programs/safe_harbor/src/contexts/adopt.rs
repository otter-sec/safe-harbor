use crate::errors::ValidationError;
use crate::states::{Adopt, AdoptUpdateType, AgreementData};
use crate::utils::events::{AdoptionUpdated, SafeHarborAdopted};
use crate::utils::types::AccountInScope;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hash;

/// Context for creating or updating an Adoption of an Agreement
///
/// ## PDA Derivation
/// The `adoption` account is a PDA that uniquely represents an adoption record
/// for a given `adopter` and `chain_id`.
///
/// PDA seeds used:
/// - `Adopt::ADOPT_SEED` (8 bytes)
/// - `adopter.key().as_ref()` (32 bytes)
/// - `chain_id_hash` (32 bytes - SHA256 hash of the CAIP-2 chain ID)
///
/// ## Why Hash the Chain ID?
/// Solana PDA seeds have a **32-byte limit per seed**.
/// We support chain IDs up to 64 characters (`MAX_CHAIN_ID_LEN = 64`), but many
/// CAIP-2 identifiers exceed the 32-byte PDA limit:
///
/// Examples that DON'T fit in 32 bytes:
/// - "bip122:000000000019d6689c085ae165831e93" (41 bytes)
/// - "solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp" (43 bytes)
/// - "polkadot:91b171bb158e2d3848fa23a9f1c25182" (44 bytes)
///
/// By hashing to SHA256, we guarantee all chain IDs fit within 32 bytes.
/// The client computes SHA256(chain_id) and passes it as `chain_id_hash`.
/// We validate on-chain that the hash matches to prevent tampering.
///
/// ## Mapping rules
/// - Each `adopter` can have **multiple adoption accounts**, one per chain (1:N mapping).
/// - This allows the same protocol to adopt an agreement on multiple chains.
///
/// ## Relationship with Agreement
/// - The `agreement` account stores the AgreementData.
/// - The adopter can adopt ANY agreement (doesn't need to own it).
/// - This allows protocols to adopt whitehat agreements created by others.
#[derive(Accounts)]
#[instruction(
    chain_id: String,
    chain_id_hash: [u8; 32],
    update_type: u8,
    new_agreement: Option<Pubkey>,
    asset_recovery_address: Option<String>,
    accounts: Option<Vec<AccountInScope>>,
    account_addresses_to_remove: Option<Vec<String>>
)]
pub struct CreateOrUpdateAdoption<'info> {
    #[account(
        init_if_needed,
        payer = adopter,
        space = Adopt::INITIAL_SPACE,
        seeds = [
            Adopt::ADOPT_SEED,
            adopter.key().as_ref(),
            chain_id_hash.as_ref()
        ],
        bump
    )]
    pub adoption: Account<'info, Adopt>,

    #[account(mut)]
    pub adopter: Signer<'info>,

    pub agreement: Account<'info, AgreementData>,
    pub system_program: Program<'info, System>,
}

impl CreateOrUpdateAdoption<'_> {
    #[allow(clippy::too_many_arguments)]
    pub fn create_or_update_adoption(
        &mut self,
        chain_id: String,
        chain_id_hash: [u8; 32],
        update_type: AdoptUpdateType,
        new_agreement: Option<Pubkey>,
        asset_recovery_address: Option<String>,
        accounts: Option<Vec<AccountInScope>>,
        account_addresses_to_remove: Option<Vec<String>>,
    ) -> Result<()> {
        // Validate that the provided hash matches the chain_id
        // This prevents clients from using an arbitrary hash to create PDAs
        // that don't correspond to the actual chain_id they claim
        let computed_hash = hash(chain_id.as_bytes());
        require!(
            computed_hash.to_bytes() == chain_id_hash,
            ValidationError::InvalidAccountParams
        );

        // Validate inputs
        require!(
            chain_id.len() <= crate::states::adopt::MAX_CHAIN_ID_LEN,
            ValidationError::MaxLengthExceeded
        );

        let adoption = &mut self.adoption;

        match update_type {
            AdoptUpdateType::InitializeOrUpdate => {
                // Full create or update - requires agreement and asset_recovery_address
                if let (Some(recovery_addr), Some(accts)) = (asset_recovery_address, accounts) {
                    require!(
                        recovery_addr.len() <= crate::states::adopt::MAX_ASSET_RECOVERY_ADDR_LEN,
                        ValidationError::MaxLengthExceeded
                    );
                    require!(
                        accts.len() <= crate::states::adopt::MAX_ACCOUNTS_PER_CHAIN,
                        ValidationError::MaxLengthExceeded
                    );

                    adoption.agreement = self.agreement.key();
                    adoption.caip2_chain_id = chain_id.clone();
                    adoption.asset_recovery_address = recovery_addr.clone();
                    adoption.accounts = accts;

                    emit!(SafeHarborAdopted {
                        adopter: self.adopter.key(),
                        chain_id: chain_id.clone(),
                        agreement: self.agreement.key(),
                        asset_recovery_address: recovery_addr,
                    });
                }
            }

            AdoptUpdateType::ReplaceAll => {
                // For replace all, we need agreement, asset_recovery_address, and accounts
                if let (Some(agreement), Some(recovery_addr), Some(accts)) =
                    (new_agreement, asset_recovery_address, accounts)
                {
                    require!(
                        recovery_addr.len() <= crate::states::adopt::MAX_ASSET_RECOVERY_ADDR_LEN,
                        ValidationError::MaxLengthExceeded
                    );
                    require!(
                        accts.len() <= crate::states::adopt::MAX_ACCOUNTS_PER_CHAIN,
                        ValidationError::MaxLengthExceeded
                    );

                    adoption.agreement = agreement;
                    adoption.asset_recovery_address = recovery_addr;
                    adoption.accounts = accts;
                }
            }

            AdoptUpdateType::AddAccounts => {
                if let Some(new_accounts) = accounts {
                    // Add new accounts to existing list
                    for new_account in new_accounts {
                        // Check for duplicates
                        if !adoption
                            .accounts
                            .iter()
                            .any(|a| a.account_address == new_account.account_address)
                        {
                            adoption.accounts.push(new_account);
                        }
                    }

                    require!(
                        adoption.accounts.len() <= crate::states::adopt::MAX_ACCOUNTS_PER_CHAIN,
                        ValidationError::MaxLengthExceeded
                    );
                }
            }

            AdoptUpdateType::RemoveAccounts => {
                if let Some(addresses) = account_addresses_to_remove {
                    // Remove accounts by address
                    adoption
                        .accounts
                        .retain(|account| !addresses.contains(&account.account_address));
                }
            }

            AdoptUpdateType::UpdateAssetRecoveryAddress => {
                if let Some(new_addr) = asset_recovery_address {
                    require!(
                        new_addr.len() <= crate::states::adopt::MAX_ASSET_RECOVERY_ADDR_LEN,
                        ValidationError::MaxLengthExceeded
                    );
                    adoption.asset_recovery_address = new_addr;
                }
            }

            AdoptUpdateType::UpdateAgreement => {
                if let Some(agreement) = new_agreement {
                    adoption.agreement = agreement;
                }
            }
        }

        emit!(AdoptionUpdated {
            adopter: self.adopter.key(),
            chain_id,
            adoption: adoption.key(),
            update_type,
        });

        Ok(())
    }
}
