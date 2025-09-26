use crate::errors::ValidationError;
use crate::states::Registry;
use crate::utils::types::{BountyTerms, Contact};
use crate::utils::Chain;
use anchor_lang::prelude::*;

/// Max constants for the agreement data
pub const MAX_CHAINS: usize = 32;
pub const MAX_CONTACTS: usize = 4;
pub const MAX_ACCOUNTS: usize = 4;
pub const MAX_CONTACT_NAME_LEN: usize = 32;
pub const MAX_CONTACT_INFO_LEN: usize = 32;
pub const MAX_PROTOCOL_NAME_LEN: usize = 64;
pub const MAX_AGREEMENT_URI_LEN: usize = 256;
pub const MAX_DILIGENCE_REQUIREMENTS_LEN: usize = 512;

/// AgreementData is used to store the data for the agreement
/// It contains the following fields:
/// - owner: The owner of the agreement
/// - protocol_name: The name of the protocol
/// - contact_details: The contact details of the protocol
/// - chains: The chains of the protocol
/// - bounty_terms: The bounty terms of the protocol
/// - agreement_uri: The agreement URI of the protocol
///
#[account]
#[derive(Default, InitSpace)]
pub struct AgreementData {
    pub owner: Pubkey,
    #[max_len(MAX_PROTOCOL_NAME_LEN)]
    pub protocol_name: String,
    #[max_len(MAX_CONTACTS)]
    pub contact_details: Vec<Contact>,
    #[max_len(MAX_CHAINS)]
    pub chains: Vec<Chain>,
    pub bounty_terms: BountyTerms,
    #[max_len(MAX_AGREEMENT_URI_LEN)]
    pub agreement_uri: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum AgreementUpdateType {
    InitializeOrUpdate,
    ProtocolName,
    ContactDetails,
    BountyTerms,
    AgreementUri,
    Chains,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum AccountUpdateType {
    Added,
    Removed,
}

impl AgreementData {
    pub const AGREEMENT_SEED: &'static [u8] = b"agreement_v2";

    /// Validates the entire agreement data before updating it
    pub fn validate_agreement_data(&self, registry: &Registry) -> Result<()> {
        self.validate_protocol_name()?;
        self.validate_bounty_terms_data()?;
        self.validate_contact_details()?;
        self.validate_agreement_uri()?;
        self.validate_chains(registry)?;

        Ok(())
    }

    /// Validates the chains before updating it in the agreement data using the registry
    pub fn validate_chains(&self, registry: &Registry) -> Result<()> {
        for chain in &self.chains {
            registry.validate_chain_id(chain.caip2_chain_id)?;
        }
        Ok(())
    }

    /// Validates the contact details before updating it in the agreement data
    pub fn validate_contact_details(&self) -> Result<()> {
        for contact in &self.contact_details {
            if contact.name.is_empty() || contact.name.trim().is_empty() {
                return Err(ValidationError::InvalidContactDetails.into());
            }
            if contact.contact.is_empty() || contact.contact.trim().is_empty() {
                return Err(ValidationError::InvalidContactDetails.into());
            }
        }
        Ok(())
    }

    /// Validates the protocol name before updating it in the agreement data
    pub fn validate_protocol_name(&self) -> Result<()> {
        if self.protocol_name.is_empty() || self.protocol_name.trim().is_empty() {
            return Err(ValidationError::InvalidProtocolName.into());
        }
        Ok(())
    }

    /// Validates the bounty terms before updating it in the agreement data
    pub fn validate_bounty_terms_data(&self) -> Result<()> {
        if self.bounty_terms.bounty_percentage > 100 || self.bounty_terms.bounty_percentage == 0 {
            return Err(ValidationError::InvalidBountyPercentage.into());
        }
        if self.bounty_terms.bounty_cap_usd == 0 {
            return Err(ValidationError::InvalidBountyCapUsd.into());
        }

        if self.bounty_terms.aggregate_bounty_cap_usd > 0 {
            return Err(ValidationError::InvalidAggregateBountyCapUsd.into());
        }

        Ok(())
    }

    /// Validates the agreement URI before updating it in the agreement data
    pub fn validate_agreement_uri(&self) -> Result<()> {
        if self.agreement_uri.is_empty() || self.agreement_uri.trim().is_empty() {
            return Err(ValidationError::InvalidAgreementUri.into());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::IdentityRequirements;

    use super::*;

    #[test]
    /// Test the validate_chains function
    fn test_validate_chains() {
        let mut registry = Registry::default();
        registry.add_valid_chains(vec![1]).unwrap();
        let agreement = AgreementData { chains: vec![Chain {
            caip2_chain_id: 1,
            asset_recovery_address: Pubkey::default(),
            accounts: vec![],
        }], ..Default::default() };
        assert!(agreement.validate_chains(&registry).is_ok());
    }

    #[test]
    /// Test the validate_contact_details function
    fn test_validate_contact_details() {
            let mut agreement = AgreementData { contact_details: vec![Contact {
                name: "Test".to_string(),
                contact: "Test".to_string(),
            }], ..Default::default() };
        assert!(agreement.validate_contact_details().is_ok());
        // test empty names are not allowed
        agreement.contact_details = vec![Contact {
            name: "".to_string(),
            contact: "Test".to_string(),
        }];
        assert!(agreement.validate_contact_details().is_err());
        // test empty contact details are not allowed
        agreement.contact_details = vec![Contact {
            name: "Test".to_string(),
            contact: "".to_string(),
        }];
        assert!(agreement.validate_contact_details().is_err());
    }

    #[test]
    /// Test the validate_protocol_name function
    fn test_validate_protocol_name() {
        let mut agreement = AgreementData { protocol_name: "Test".to_string(), ..Default::default() };
        assert!(agreement.validate_protocol_name().is_ok());
        // test empty protocol names are not allowed
        agreement.protocol_name = "".to_string();
        assert!(agreement.validate_protocol_name().is_err());
        // test empty protocol names are not allowed
        agreement.protocol_name = " ".to_string();
        assert!(agreement.validate_protocol_name().is_err());
    }

    #[test]
    /// Test the validate_bounty_terms_data function
    fn test_validate_bounty_terms_data() {
        let mut agreement = AgreementData { bounty_terms: BountyTerms {
            bounty_percentage: 100,
            bounty_cap_usd: 1000000,
            aggregate_bounty_cap_usd: 1000000,
            retainable: true,
            identity: IdentityRequirements::Anonymous,
            diligence_requirements: "Test".to_string(),
        }, ..Default::default() };
        assert!(agreement.validate_bounty_terms_data().is_ok());
        // test invalid bounty percentage
        agreement.bounty_terms.bounty_percentage = 101;
        assert!(agreement.validate_bounty_terms_data().is_err());
        // test invalid bounty cap usd
        agreement.bounty_terms.bounty_cap_usd = 0;
        assert!(agreement.validate_bounty_terms_data().is_err());
        // test invalid aggregate bounty cap usd
        agreement.bounty_terms.aggregate_bounty_cap_usd = 1000000;
        assert!(agreement.validate_bounty_terms_data().is_err());
        // test invalid identity
        agreement.bounty_terms.identity = IdentityRequirements::Named;
        assert!(agreement.validate_bounty_terms_data().is_err());
        // test empty diligence requirements
        agreement.bounty_terms.diligence_requirements = "".to_string();
        assert!(agreement.validate_bounty_terms_data().is_err());
    }
}
