use crate::errors::{BountyError, ChainError, ValidationError};
use crate::states::Registry;
use crate::utils::types::{BountyTerms, Chain, Contact};
use anchor_lang::prelude::*;

pub const MAX_CONTACTS: usize = 16;
pub const MAX_CONTACT_NAME_LEN: usize = 64;
pub const MAX_CONTACT_INFO_LEN: usize = 64;
pub const MAX_PROTOCOL_NAME_LEN: usize = 64;
pub const MAX_AGREEMENT_URI_LEN: usize = 128;

pub const MAX_AGREEMENT_CHAINS: usize = 8;
pub const MAX_ACCOUNTS_PER_CHAIN: usize = 64;
pub const MAX_ACCOUNT_ADDR_LEN: usize = 40;
pub const MAX_CHAIN_ID_LEN: usize = 64;
pub const MAX_ASSET_RECOVERY_ADDR_LEN: usize = 64;

#[account]
#[derive(Default)]
pub struct AgreementData {
    pub owner: Pubkey,

    pub protocol_name: String,
    pub contact_details: Vec<Contact>,
    pub chains: Vec<Chain>,
    pub bounty_terms: BountyTerms,
    pub agreement_uri: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum AgreementUpdateType {
    ProtocolName,
    ContactDetails,
    Chains,
    BountyTerms,
    AgreementUri,
    InitializeOrUpdate,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum ChainUpdateType {
    Added,
    Updated,
    Removed,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum AccountUpdateType {
    Added,
    Removed,
}

impl AgreementData {
    pub const INITIAL_SPACE: usize = 8  // discriminator
    + 32 // owner
    + 4 + MAX_PROTOCOL_NAME_LEN
    + 4 + (MAX_CONTACTS * (4 + MAX_CONTACT_NAME_LEN + 4 + MAX_CONTACT_INFO_LEN))
    + 4 + (MAX_AGREEMENT_CHAINS * (
        4 + MAX_ASSET_RECOVERY_ADDR_LEN +
        4 + (MAX_ACCOUNTS_PER_CHAIN * (4 + MAX_ACCOUNT_ADDR_LEN + 1)) + // account + enum
        4 + MAX_CHAIN_ID_LEN
    ))
    + (8 + 8 + 1 + 1 + 4 + 256 + 8) // BountyTerms (approx)
    + 4 + MAX_AGREEMENT_URI_LEN + 1024;

    /// Validates the entire agreement data
    pub fn validate_agreement_data(&self, registry: &Registry) -> Result<()> {
        self.validate_protocol_name()?;
        self.validate_chains(registry)?;
        self.validate_bounty_terms_data()?;
        self.validate_contact_details()?;
        self.validate_agreement_uri()?;

        self.validate_no_duplicate_chain_ids()?;
        Ok(())
    }
    pub fn validate_contact_details(&self) -> Result<()> {
        for contact in &self.contact_details {
            if contact.name.len() > MAX_CONTACT_NAME_LEN {
                return Err(ValidationError::InvalidContactDetails.into());
            }
            if contact.contact.len() > MAX_CONTACT_INFO_LEN {
                return Err(ValidationError::InvalidContactDetails.into());
            }
        }
        if self.contact_details.len() > MAX_CONTACTS {
            return Err(ValidationError::InvalidContactDetails.into());
        }
        Ok(())
    }

    pub fn validate_no_duplicate_chain_ids(&self) -> Result<()> {
        let mut seen_chain_ids = std::collections::HashSet::new();
        for chain in &self.chains {
            if !seen_chain_ids.insert(&chain.caip2_chain_id) {
                return Err(ChainError::DuplicateChainId.into());
            }
        }
        Ok(())
    }

    pub fn validate_protocol_name(&self) -> Result<()> {
        if self.protocol_name.is_empty() {
            return Err(ValidationError::InvalidProtocolName.into());
        }

        if self.protocol_name.len() > MAX_PROTOCOL_NAME_LEN {
            return Err(ValidationError::InvalidProtocolName.into());
        }

        if self.protocol_name.trim().is_empty() {
            return Err(ValidationError::InvalidProtocolName.into());
        }

        Ok(())
    }

    pub fn validate_chains(&self, registry: &Registry) -> Result<()> {
        for chain in &self.chains {
            if !registry.valid_chains.contains(&chain.caip2_chain_id) {
                return Err(ChainError::ChainNotValidInRegistry.into());
            }
            if chain.caip2_chain_id.len() > MAX_CHAIN_ID_LEN {
                return Err(ChainError::MaxLengthExceeded.into());
            }
            if chain.asset_recovery_address.len() > MAX_ASSET_RECOVERY_ADDR_LEN {
                return Err(ChainError::MaxLengthExceeded.into());
            }
            if chain.accounts.len() > MAX_ACCOUNTS_PER_CHAIN {
                return Err(ChainError::MaxLengthExceeded.into());
            }
            for account in &chain.accounts {
                if account.account_address.len() > MAX_ACCOUNT_ADDR_LEN {
                    return Err(ChainError::MaxLengthExceeded.into());
                }
            }
            if chain.accounts.len() > MAX_ACCOUNTS_PER_CHAIN {
                return Err(ChainError::MaxLengthExceeded.into());
            }
        }

        Ok(())
    }

    pub fn validate_bounty_terms_data(&self) -> Result<()> {
        if self.bounty_terms.bounty_percentage > 100 {
            return Err(BountyError::InvalidBountyPercentage.into());
        }

        if self.bounty_terms.aggregate_bounty_cap_usd > 0 && self.bounty_terms.retainable {
            return Err(BountyError::CannotSetBothAggregateBountyCapUSDAndRetainable.into());
        }

        Ok(())
    }

    pub fn validate_agreement_uri(&self) -> Result<()> {
        if self.agreement_uri.len() > MAX_AGREEMENT_URI_LEN {
            return Err(ValidationError::InvalidAgreementUri.into());
        }
        Ok(())
    }
}
