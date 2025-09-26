use anchor_lang::prelude::*;

use crate::states::*;
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, InitSpace)]
pub struct Contact {
    #[max_len(MAX_CONTACT_NAME_LEN)]
    pub name: String,
    #[max_len(MAX_CONTACT_INFO_LEN)]
    pub contact: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Chain {
    pub asset_recovery_address: Pubkey,
    #[max_len(MAX_ACCOUNTS)]
    pub accounts: Vec<AccountInScope>,
    pub caip2_chain_id: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, InitSpace)]
pub struct AccountInScope {
    pub account_address: Pubkey,
    pub child_contract_scope: ChildContractScope,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
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
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
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
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, InitSpace)]
pub struct BountyTerms {
    pub bounty_percentage: u64,
    pub bounty_cap_usd: u64,
    pub retainable: bool,
    pub identity: IdentityRequirements,
    #[max_len(MAX_DILIGENCE_REQUIREMENTS_LEN)]
    pub diligence_requirements: String,
    pub aggregate_bounty_cap_usd: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_values() {
        let contact: Contact = Default::default();
        assert_eq!(contact.name, "");

        let bounty: BountyTerms = Default::default();
        assert_eq!(bounty.aggregate_bounty_cap_usd, 0);
    }

    #[test]
    fn test_bounty_terms_validation() {
        let bounty = BountyTerms {
            bounty_percentage: 101,
            ..Default::default()
        };
        assert_eq!(bounty.bounty_percentage, 101);
    }

    #[test]
    fn test_child_contract_scope() {
        let scope = ChildContractScope::ExistingOnly;

        let account = AccountInScope {
            account_address: Pubkey::default(),
            child_contract_scope: scope.clone(),
        };

        // serialization
        let serialized = account.try_to_vec().unwrap();
        let deserialized: AccountInScope = AccountInScope::try_from_slice(&serialized).unwrap();

        // Compare by serializing both and checking equality
        let original_serialized = account.try_to_vec().unwrap();
        let deserialized_serialized = deserialized.try_to_vec().unwrap();
        assert_eq!(original_serialized, deserialized_serialized);
    }
}
