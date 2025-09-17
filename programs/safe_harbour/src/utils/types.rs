use anchor_lang::prelude::*;
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Contact {
    pub name: String,
    pub contact: String,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct AccountInScope {
    pub account_address: String,
    pub child_contract_scope: ChildContractScope,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Chain {
    pub asset_recovery_address: String,
    pub accounts: Vec<AccountInScope>,
    pub caip2_chain_id: String,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum ChildContractScope {
    None,
    ExistingOnly,
    All,
    FutureOnly,
}

impl Default for ChildContractScope {
    fn default() -> Self {
        Self::None
    }
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum IdentityRequirements {
    Anonymous,
    Pseudonymous,
    Named,
}

impl Default for IdentityRequirements {
    fn default() -> Self {
        Self::Anonymous
    }
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct BountyTerms {
    pub bounty_percentage: u64,
    pub bounty_cap_usd: u64,
    pub retainable: bool,
    pub identity: IdentityRequirements,
    pub diligence_requirements: String,
    pub aggregate_bounty_cap_usd: u64,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct AgreementData {
    pub protocol_name: String,
    pub contact_details: Vec<Contact>,
    pub chains: Vec<Chain>,
    pub bounty_terms: BountyTerms,
    pub agreement_uri: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_values() {
        // Test that default values are sensible
        let contact: Contact = Default::default();
        assert_eq!(contact.name, "");
        assert_eq!(contact.contact, "");

        let account: AccountInScope = Default::default();
        assert_eq!(account.account_address, "");
        match account.child_contract_scope {
            ChildContractScope::None => {}
            _ => panic!("default child scope should be None"),
        }

        let identity: IdentityRequirements = Default::default();
        match identity {
            IdentityRequirements::Anonymous => {}
            _ => panic!("default identity should be Anonymous"),
        }

        let bounty: BountyTerms = Default::default();
        assert_eq!(bounty.bounty_percentage, 0);
        assert_eq!(bounty.bounty_cap_usd, 0);
        assert_eq!(bounty.retainable, false);
        assert_eq!(bounty.diligence_requirements, "");
        assert_eq!(bounty.aggregate_bounty_cap_usd, 0);

        let agreement: AgreementData = Default::default();
        assert_eq!(agreement.protocol_name, "");
        assert_eq!(agreement.contact_details.len(), 0);
        assert_eq!(agreement.chains.len(), 0);
        assert_eq!(agreement.agreement_uri, "");
    }

    #[test]
    fn test_serialization() {
        // Test that all types can be serialized and deserialized
        let contact = Contact {
            name: "John Doe".to_string(),
            contact: "john@example.com".to_string(),
        };

        let account = AccountInScope {
            account_address: "0x1234567890123456789012345678901234567890".to_string(),
            child_contract_scope: ChildContractScope::All,
        };

        let chain = Chain {
            asset_recovery_address: "0x9876543210987654321098765432109876543210".to_string(),
            accounts: vec![account],
            caip2_chain_id: "eip155:1".to_string(),
        };

        let bounty_terms = BountyTerms {
            bounty_percentage: 10,
            bounty_cap_usd: 100000,
            retainable: false,
            identity: IdentityRequirements::Pseudonymous,
            diligence_requirements: "Must provide proof of identity".to_string(),
            aggregate_bounty_cap_usd: 500000,
        };

        let agreement = AgreementData {
            protocol_name: "Test Protocol".to_string(),
            contact_details: vec![contact],
            chains: vec![chain],
            bounty_terms,
            agreement_uri: "ipfs://QmTestHash".to_string(),
        };

        // Test serialization
        let serialized = agreement.try_to_vec().unwrap();
        let deserialized: AgreementData = AgreementData::try_from_slice(&serialized).unwrap();

        assert_eq!(agreement.protocol_name, deserialized.protocol_name);
        assert_eq!(
            agreement.contact_details.len(),
            deserialized.contact_details.len()
        );
        assert_eq!(agreement.chains.len(), deserialized.chains.len());
        assert_eq!(agreement.agreement_uri, deserialized.agreement_uri);
    }

    #[test]
    fn test_bounty_terms_validation() {
        // Test bounty percentage bounds
        let mut bounty = BountyTerms::default();
        bounty.bounty_percentage = 101; // Should be valid (no bounds checking in struct)
        assert_eq!(bounty.bounty_percentage, 101);

        // Test retainable flag
        bounty.retainable = true;
        assert!(bounty.retainable);

        // Test identity requirements
        bounty.identity = IdentityRequirements::Named;
        match bounty.identity {
            IdentityRequirements::Named => {}
            _ => panic!("Identity should be Named"),
        }
    }

    #[test]
    fn test_child_contract_scope() {
        // Test all child contract scope variants
        let scopes = vec![
            ChildContractScope::None,
            ChildContractScope::ExistingOnly,
            ChildContractScope::All,
            ChildContractScope::FutureOnly,
        ];

        for scope in scopes {
            let account = AccountInScope {
                account_address: "0x123".to_string(),
                child_contract_scope: scope.clone(),
            };

            // Test serialization
            let serialized = account.try_to_vec().unwrap();
            let deserialized: AccountInScope = AccountInScope::try_from_slice(&serialized).unwrap();

            // Compare by serializing both and checking equality
            let original_serialized = account.try_to_vec().unwrap();
            let deserialized_serialized = deserialized.try_to_vec().unwrap();
            assert_eq!(original_serialized, deserialized_serialized);
        }
    }
}
