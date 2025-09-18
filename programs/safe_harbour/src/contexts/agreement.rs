use crate::states::{AgreementData, AgreementUpdateType};
use crate::utils::events::AgreementUpdated;
use anchor_lang::prelude::*;

/// Context for creating or updating a Safe Harbor agreement
#[derive(Accounts)]
#[instruction(init_nonce: u64)]
pub struct CreateOrUpdateAgreement<'info> {
    #[account(
        init_if_needed,
        seeds=[AgreementData::AGREEMENT_SEED, signer.key().as_ref(), init_nonce.to_le_bytes().as_ref()],
        payer = signer,
        space = AgreementData::INITIAL_SPACE,
        bump,
    )]
    pub agreement: Account<'info, AgreementData>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl CreateOrUpdateAgreement<'_> {
    pub fn create_or_update_agreement(
        &mut self,
        data: AgreementData,
        owner: Pubkey,
        update_type: AgreementUpdateType,
    ) -> Result<()> {
        let agreement = &mut self.agreement;
        data.validate_agreement_data()?;

        match update_type {
            AgreementUpdateType::ProtocolName => {
                agreement.protocol_name = data.protocol_name;
            }
            AgreementUpdateType::ContactDetails => {
                agreement.contact_details = data.contact_details;
            }
            AgreementUpdateType::BountyTerms => {
                agreement.bounty_terms = data.bounty_terms;
            }

            AgreementUpdateType::AgreementUri => {
                agreement.agreement_uri = data.agreement_uri;
            }

            AgreementUpdateType::InitializeOrUpdate => {
                agreement.owner = owner;
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
