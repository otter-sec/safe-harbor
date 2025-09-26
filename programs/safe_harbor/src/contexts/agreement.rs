use crate::states::registry::Registry;
use crate::states::{AgreementData, AgreementUpdateType};
use crate::utils::events::AgreementUpdated;
use anchor_lang::prelude::*;

/// Context for creating or updating a Safe Harbor agreement
///
/// ## PDA Derivation
/// The `agreement` account is a Program Derived Address (PDA) that uniquely represents
/// an agreement adopted by a owner.  
///
/// PDA seeds used:
/// - `AgreementData::AGREEMENT_SEED`
/// - `signer.key().as_ref()` (the user who is creating the agreement)
/// - `init_nonce.to_le_bytes()` (a unique nonce to allow multiple agreements per signer)
///
/// ## Why `init_nonce`?
/// - Without `init_nonce`, a user can only create **one** agreement, because the PDA
///   would always be derived from `[AGREEMENT_SEED, signer.key()]`.
/// - In the EVM version, users are allowed to create multiple agreements. To maintain
///   consistency with that model, we include `init_nonce` as an extra seed.
/// - This makes it possible for the same `signer` to create multiple unique agreements,
///   while still ensuring deterministic PDA derivation.
///
/// ## Mapping rules
/// - Multiple agreements can belong to the same user (`1:N` relationship).
#[derive(Accounts)]
#[instruction(init_nonce: u64)]
pub struct CreateOrUpdateAgreement<'info> {
    #[account(
        init_if_needed,
        seeds = [
            AgreementData::AGREEMENT_SEED,
            signer.key().as_ref(),
            init_nonce.to_le_bytes().as_ref(),
        ],
        payer = signer,
        space = 8 + AgreementData::INIT_SPACE,
        bump,
    )]
    pub agreement: Account<'info, AgreementData>,

    #[account(
        mut,
        seeds=[Registry::REGISTRY_SEED],
        bump
    )]
    pub registry: Account<'info, Registry>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl CreateOrUpdateAgreement<'_> {
    /// Creates or updates an agreement with the given data and update type.
    pub fn create_or_update_agreement(
        &mut self,
        data: AgreementData,
        owner: Pubkey,
        update_type: AgreementUpdateType,
    ) -> Result<()> {
        let agreement = &mut self.agreement;

        match update_type {
            // Update the protocol name
            AgreementUpdateType::ProtocolName => {
                agreement.validate_protocol_name()?;
                agreement.protocol_name = data.protocol_name;
            }
            // Update the contact details
            AgreementUpdateType::ContactDetails => {
                agreement.validate_contact_details()?;
                agreement.contact_details = data.contact_details;
            }
            // Update the bounty terms
            AgreementUpdateType::BountyTerms => {
                agreement.validate_bounty_terms_data()?;
                agreement.bounty_terms = data.bounty_terms;
            }
            // Update the agreement URI
            AgreementUpdateType::AgreementUri => {
                agreement.validate_agreement_uri()?;
                agreement.agreement_uri = data.agreement_uri;
            }
            // Update the chains
            AgreementUpdateType::Chains => {
                agreement.validate_chains(&self.registry)?;
                agreement.chains = data.chains;
            }
            // Initialize or update the whole agreement data
            AgreementUpdateType::InitializeOrUpdate => {
                agreement.validate_agreement_data(&self.registry)?;
                agreement.owner = owner;
                agreement.chains = data.chains;
                agreement.agreement_uri = data.agreement_uri;
                agreement.protocol_name = data.protocol_name;
                agreement.contact_details = data.contact_details;
                agreement.bounty_terms = data.bounty_terms;
            }
        }

        emit!(AgreementUpdated {
            agreement: agreement.key(),
            update_type,
        });

        Ok(())
    }
}
