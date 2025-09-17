#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

declare_id!("8wRWiR6DFVP3JmW8i4bTa393PorrTXsQhrZgdfQjLpmX");

pub mod constants;
pub mod contexts;
pub mod errors;
pub mod states;
pub mod utils;

pub use states::{AgreementData, AgreementUpdateType};
pub use contexts::*;

#[program]
pub mod safe_harbour {
    use super::*;

    /// Initializes the global registry PDA.
    pub fn initialize(ctx: Context<Initialize>, valid_chains: Vec<String>) -> Result<()> {
        ctx.accounts.initialize(valid_chains)
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
        data: AgreementData,
        owner: Pubkey,
        update_type: AgreementUpdateType,
        init_nonce: u64,
    ) -> Result<()> {
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
