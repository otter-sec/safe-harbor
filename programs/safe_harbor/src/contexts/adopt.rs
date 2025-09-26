use crate::states::{Adopt, AgreementData};
use crate::utils::events::SafeHarborAdopted;
use anchor_lang::prelude::*;

/// Context for creating or updating an Adoption of an Agreement
///
/// ## PDA Derivation
/// The `adoption` account is a PDA that uniquely represents an adoption record
/// for a given `owner`.
///
/// PDA seeds used:
/// - `Adopt::ADOPT_SEED`
/// - `owner.key().as_ref()` (the user who is adopting)
///
/// ## Mapping rules
/// - Each `owner` can have **at most one adoption account** (1:1 mapping).
/// - This ensures that adoption is unique per user and prevents multiple active
///   adoptions at the same time.
///
/// ## Relationship with Agreement
/// - The `agreement` account stores the AgreementData.
/// - `#[account(has_one = owner)]` enforces that the adoption must be tied
///   to an agreement whose `owner` parameter matches the `owner` signer in this context.
/// - This guarantees consistency: only the rightful owner can create/update
///   an adoption record for their agreement.
#[derive(Accounts)]
pub struct CreateOrUpdateAdoption<'info> {
    #[account(
        init_if_needed,
        payer = owner,
        space = 8 + Adopt::INIT_SPACE,
        seeds = [Adopt::ADOPT_SEED, owner.key().as_ref()],
        bump
    )]
    pub adoption: Account<'info, Adopt>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(has_one = owner)]
    pub agreement: Account<'info, AgreementData>,
    pub system_program: Program<'info, System>,
}
impl CreateOrUpdateAdoption<'_> {
    /// Creates or updates an adoption record for the given agreement.
    pub fn create_or_update_adoption(&mut self) -> Result<()> {
        let old_agreement = self.adoption.agreement.key();

        let adoption = &mut self.adoption;
        // Set the new agreement key in the adoption record
        adoption.agreement = self.agreement.key();

        emit!(SafeHarborAdopted {
            adopter: self.owner.key(),
            old_agreement,
            new_agreement: self.agreement.key(),
        });

        Ok(())
    }
}
