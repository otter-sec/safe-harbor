#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

declare_id!("CxQZFj98gyJSJhBh4kZ6jCpQcaW1gcx9gcwcCr2KrkNH");

pub mod constants;
pub mod contexts;
pub mod errors;
pub mod states;
pub mod utils;

use contexts::*;
pub use states::{Adopt, Agreement, Registry};
pub use utils::types::{
    AccountInScope, AgreementData, BountyTerms, Chain, ChildContractScope, Contact,
    IdentityRequirements,
};

#[program]
pub mod safe_harbour {
    use super::*;

    /// Creates a new agreement account or updates an existing one with the specified parameters.
    /// The agreement can be modified by its owner after creation.
    ///
    /// # Arguments
    /// * `ctx` - The creation/update context
    /// * `data` - The agreement data
    /// * `owner` - The public key of the agreement owner
    ///
    /// # Returns
    /// * `Result<()>` - Success or error
    pub fn create_or_update_agreement(
        ctx: Context<CreateOrUpdateAgreement>,
        data: AgreementData,
        owner: Pubkey,
    ) -> Result<()> {
        contexts::agreement::create_or_update_agreement(ctx, data, owner)
    }

    /// Creates or updates an adoption entry where the signer adopts the specified agreement.
    /// This registers the adoption in the registry for efficient lookup.
    ///
    /// # Arguments
    /// * `ctx` - The adoption context
    ///
    /// # Returns
    /// * `Result<()>` - Success or error
    pub fn create_or_update_adoption(ctx: Context<CreateOrUpdateAdoption>) -> Result<()> {
        contexts::adoption::create_or_update_adoption(ctx)
    }
}
