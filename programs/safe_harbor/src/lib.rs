#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

declare_id!("6Q1MaYkpJCo7Kyz6dwFkN5rypTpVsmZSzqXvwtfrU1Ty");

pub mod contexts;
pub mod errors;
pub mod states;
pub mod utils;

pub use contexts::*;
pub use errors::*;
pub use states::{AdoptUpdateType, AgreementData, AgreementUpdateType};

#[program]
pub mod safe_harbor {
    use super::*;

    /// Creates a new agreement account or updates an existing one with the specified parameters.
    /// The agreement can be modified by its owner after creation.
    ///
    /// # Arguments
    /// * `data` - The agreement data
    /// * `owner` - The public key of the agreement owner
    ///
    /// # Returns
    /// * `Result<()>` - Success or error
    #[allow(unused_variables)]
    pub fn create_or_update_agreement(
        ctx: Context<CreateOrUpdateAgreement>,
        init_nonce: u64,
        data: AgreementData,
        owner: Pubkey,
        update_type: u8,
    ) -> Result<()> {
        let update_type = match update_type {
            0 => AgreementUpdateType::InitializeOrUpdate,
            1 => AgreementUpdateType::ProtocolName,
            2 => AgreementUpdateType::ContactDetails,
            3 => AgreementUpdateType::BountyTerms,
            4 => AgreementUpdateType::AgreementUri,
            _ => return Err(ValidationError::InvalidUpdateType.into()),
        };
        ctx.accounts
            .create_or_update_agreement(data, owner, update_type)
    }

    /// Creates or updates an adoption entry where the signer adopts the specified agreement
    /// for a specific chain. This allows protocols to adopt agreements on multiple chains.
    ///
    /// # Arguments
    /// * `chain_id` - The CAIP-2 chain ID (e.g., "eip155:1" for Ethereum mainnet)
    /// * `chain_id_hash` - SHA256 hash of the chain_id (used for PDA derivation due to 32-byte seed limit)
    /// * `update_type` - Type of update to perform (0-5)
    /// * `new_agreement` - Optional: new agreement to adopt (required for ReplaceAll)
    /// * `asset_recovery_address` - Optional: recovery address (required for InitializeOrUpdate, ReplaceAll, UpdateAssetRecoveryAddress)
    /// * `accounts` - Optional: accounts list (required for InitializeOrUpdate, ReplaceAll, AddAccounts)
    /// * `account_addresses_to_remove` - Optional: account addresses to remove (required for RemoveAccounts)
    ///
    /// # Returns
    /// * `Result<()>` - Success or error
    #[allow(clippy::too_many_arguments)]
    pub fn create_or_update_adoption(
        ctx: Context<CreateOrUpdateAdoption>,
        chain_id: String,
        chain_id_hash: [u8; 32],
        update_type: u8,
        new_agreement: Option<Pubkey>,
        asset_recovery_address: Option<String>,
        accounts: Option<Vec<crate::utils::types::AccountInScope>>,
        account_addresses_to_remove: Option<Vec<String>>,
    ) -> Result<()> {
        let update_type = match update_type {
            0 => AdoptUpdateType::InitializeOrUpdate,
            1 => AdoptUpdateType::ReplaceAll,
            2 => AdoptUpdateType::AddAccounts,
            3 => AdoptUpdateType::RemoveAccounts,
            4 => AdoptUpdateType::UpdateAssetRecoveryAddress,
            5 => AdoptUpdateType::UpdateAgreement,
            _ => return Err(ValidationError::InvalidUpdateType.into()),
        };

        ctx.accounts.create_or_update_adoption(
            chain_id,
            chain_id_hash,
            update_type,
            new_agreement,
            asset_recovery_address,
            accounts,
            account_addresses_to_remove,
        )
    }
}
