use crate::states::{Adopt, Registry};
use crate::utils::events::SafeHarborAdopted;
use anchor_lang::prelude::*;

/// Context for creating or updating an adoption entry
#[derive(Accounts)]
pub struct CreateOrUpdateAdoption<'info> {
    #[account(mut, seeds=[b"registry_v2"], bump)]
    pub registry: Account<'info, Registry>,

    #[account(mut)]
    pub adopter: Signer<'info>,

    #[account(
        init_if_needed,
        payer = adopter,
        space = Adopt::SPACE,
        seeds = [b"adoption_v2", adopter.key().as_ref(), agreement.key().as_ref()],
        bump
    )]
    pub adoption: Account<'info, Adopt>,

    /// The agreement being adopted (unchecked account)
    /// CHECK: agreement PDA or normal account; stored as pubkey only
    pub agreement: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create_or_update_adoption(ctx: Context<CreateOrUpdateAdoption>) -> Result<()> {
    let adopter = ctx.accounts.adopter.key();
    let agreement = ctx.accounts.agreement.key();
    let registry = &mut ctx.accounts.registry;

    // Validate the agreement is not default
    require!(
        agreement != Pubkey::default(),
        crate::errors::ValidationError::InvalidAccountParams
    );

    let old_agreement = registry.get_agreement(adopter).unwrap_or(Pubkey::default());
    registry.set_agreement(adopter, agreement);

    let adoption = &mut ctx.accounts.adoption;
    adoption.agreement = agreement;

    // Validate the adoption after setting
    adoption.validate_agreement()?;

    emit!(SafeHarborAdopted::new(
        adopter,
        old_agreement,
        agreement,
        ctx.accounts.registry.key()
    ));

    Ok(())
}
