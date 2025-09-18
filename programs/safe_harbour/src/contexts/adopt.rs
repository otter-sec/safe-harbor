use crate::states::{Adopt,AgreementData};
use crate::utils::events::SafeHarborAdopted;
use anchor_lang::prelude::*;

/// Context for creating or updating an adoption entry
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
    pub fn create_or_update_adoption(&mut self) -> Result<()> {
        let old_agreement = self.adoption.agreement.key();

        let adoption = &mut self.adoption;
        adoption.agreement = self.agreement.key();

        emit!(SafeHarborAdopted {
            adopter: self.owner.key(),
            old_agreement,
            new_agreement: self.agreement.key(),
        });

        Ok(())
    }
}
