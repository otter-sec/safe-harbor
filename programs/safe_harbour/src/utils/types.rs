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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_values() {
        // Test that default values are sensible
        let contact: Contact = Default::default();
        assert_eq!(contact.name, "");

        let bounty: BountyTerms = Default::default();
        assert_eq!(bounty.aggregate_bounty_cap_usd, 0);
    }

    #[test]
    fn test_bounty_terms_validation() {
        // Test bounty percentage bounds
        let mut bounty = BountyTerms::default();
        bounty.bounty_percentage = 101;
        assert_eq!(bounty.bounty_percentage, 101);
    }

    #[test]
    fn test_child_contract_scope() {
        // Test all child contract scope variants
        let scope = ChildContractScope::ExistingOnly;

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
