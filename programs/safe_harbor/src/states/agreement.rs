use crate::errors::{BountyError, ValidationError};
use crate::utils::types::{BountyTerms, Contact};
use anchor_lang::prelude::*;

// Maximum number of contact entries per agreement
// Allows multiple points of contact (e.g., security team, legal, technical lead)
pub const MAX_CONTACTS: usize = 5;

// Maximum length for contact name (e.g., "Security Team", "Bug Bounty Lead")
pub const MAX_CONTACT_NAME_LEN: usize = 64;

// Maximum length for contact info (email, Telegram handle, Discord, etc.)
// Examples: email addresses, social media handles, URLs
pub const MAX_CONTACT_INFO_LEN: usize = 64;

// Maximum length for protocol name (e.g., "Raydium", "Kamino")
pub const MAX_PROTOCOL_NAME_LEN: usize = 64;

// Agreement URI max length (typically IPFS/Arweave URLs)
// Examples: IPFS CIDv1 with path ~130-150 chars, Arweave ~100-150 chars
// 256 provides sufficient headroom while minimizing rent costs
pub const MAX_AGREEMENT_URI_LEN: usize = 256;

/// Safe Harbor Agreement data stored on-chain.
/// Defines the terms and conditions for whitehat security researchers.
#[account]
#[derive(Default)]
pub struct AgreementData {
    /// The public key that owns and can update this agreement
    /// Only this authority can modify the agreement terms
    pub owner: Pubkey,

    /// The name of the protocol adopting Safe Harbor (e.g., "Raydium", "Kamino")
    /// Used for identification and display purposes
    pub protocol_name: String,

    /// List of contact points for security disclosures
    /// Can include security team emails, Telegram handles, Discord contacts, etc.
    pub contact_details: Vec<Contact>,

    /// Bounty terms defining rewards and requirements for whitehats
    /// Includes percentage, caps, identity requirements, and diligence standards
    pub bounty_terms: BountyTerms,

    /// URI pointing to the full legal agreement document
    /// Typically an IPFS or Arweave URL containing the complete terms
    pub agreement_uri: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum AgreementUpdateType {
    InitializeOrUpdate,
    ProtocolName,
    ContactDetails,
    BountyTerms,
    AgreementUri,
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
    + 4 + (8 + 8 + 1 + 1 + 4 + 256 + 8) // BountyTerms (approx)
    + 4 + MAX_AGREEMENT_URI_LEN + 1024;

    pub const AGREEMENT_SEED: &'static [u8] = b"agreement_v2";

    /// Validates the entire agreement data
    pub fn validate_agreement_data(&self) -> Result<()> {
        self.validate_protocol_name()?;
        self.validate_bounty_terms_data()?;
        self.validate_contact_details()?;
        self.validate_agreement_uri()?;

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

    pub fn validate_bounty_terms_data(&self) -> Result<()> {
        if self.bounty_terms.bounty_percentage > 100 {
            return Err(BountyError::InvalidBountyPercentage.into());
        }

        // Cannot set both aggregate_bounty_cap_usd and retainable
        // aggregate_bounty_cap_usd != 0 means it's set (0 is the default/unset value)
        if self.bounty_terms.aggregate_bounty_cap_usd != 0 && self.bounty_terms.retainable {
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
