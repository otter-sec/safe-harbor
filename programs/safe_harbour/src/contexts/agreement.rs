use crate::states::{Agreement, Registry};
use crate::utils::events::AgreementCreated;
use crate::utils::types::AgreementData;
use crate::utils::utils::*;
use crate::errors::*;
use anchor_lang::prelude::*;

/// Context for creating or updating a Safe Harbor agreement
#[derive(Accounts)]
pub struct CreateOrUpdateAgreement<'info> {
    #[account(mut, seeds=[b"registry_v2"], bump)]
    pub registry: Account<'info, Registry>,

    #[account(
        init_if_needed,
        payer = payer,
        space = Agreement::INITIAL_SPACE,
    )]
    pub agreement: Account<'info, Agreement>,

    /// The owner of the agreement (unchecked account)
    /// CHECK: Owner can be any valid pubkey, doesn't need to sign for creation
    pub owner: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create_or_update_agreement(
    ctx: Context<CreateOrUpdateAgreement>,
    data: AgreementData,
    owner: Pubkey,
) -> Result<()> {
    require!(
        is_valid_pubkey(&owner),
        ValidationError::InvalidNewOwner
    );

    // Create a temporary agreement instance for validation
    let temp_agreement = Agreement {
        owner,
        data: data.clone(),
    };

    temp_agreement.validate_agreement_data(&ctx.accounts.registry)?;

    let agreement = &mut ctx.accounts.agreement;
    agreement.owner = owner;
    agreement.data = data.clone();

    emit!(AgreementCreated::new(
        ctx.accounts.agreement.key(),
        owner,
        data.protocol_name
    ));

    Ok(())
}
