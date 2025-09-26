#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

declare_id!("6Q1MaYkpJCo7Kyz6dwFkN5rypTpVsmZSzqXvwtfrU1Ty");

pub mod contexts;
pub mod errors;
pub mod states;
pub mod utils;

pub use contexts::*;
pub use errors::*;
pub use states::{AgreementData, AgreementUpdateType};
pub use utils::types::Chain;

#[program]
pub mod safe_harbor {
    use super::*;

    /// Initializes the global registry PDA.
    pub fn initialize(ctx: Context<Initialize>, valid_chains: Vec<u64>) -> Result<()> {
        ctx.accounts.initialize(valid_chains)
    }

    /// Sets the valid chains for the registry.
    /// # Arguments
    /// * `valid_chains` - The valid chains to set
    /// # Returns
    /// * `Result<()>` - Success or error
    pub fn set_valid_chains(ctx: Context<Initialize>, valid_chains: Vec<u64>) -> Result<()> {
        ctx.accounts.set_valid_chains(valid_chains)
    }

    /// Sets the invalid chains for the registry.
    /// # Arguments
    /// * `invalid_chains` - The invalid chains to set
    /// # Returns
    /// * `Result<()>` - Success or error
    pub fn set_invalid_chains(ctx: Context<Initialize>, invalid_chains: Vec<u64>) -> Result<()> {
        ctx.accounts.set_invalid_chains(invalid_chains)
    }

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

    /// Creates or updates an adoption entry where the signer adopts the specified agreement.
    /// This registers the adoption in the registry for efficient lookup.
    ///
    /// # Returns
    /// * `Result<()>` - Success or error
    pub fn create_or_update_adoption(ctx: Context<CreateOrUpdateAdoption>) -> Result<()> {
        ctx.accounts.create_or_update_adoption()
    }
}
