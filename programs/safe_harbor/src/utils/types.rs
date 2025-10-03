use anchor_lang::prelude::*;

/// Contact information for security disclosures
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Contact {
    /// Name or role of the contact (e.g., "Security Team", "Bug Bounty Lead")
    pub name: String,

    /// Contact method (email, Telegram handle, Discord, URL, etc.)
    pub contact: String,
}

/// Represents a smart contract account that is in-scope for the Safe Harbor agreement
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct AccountInScope {
    /// The contract address (format varies by chain: 0x... for EVM, base58 for Solana, etc.)
    pub account_address: String,

    /// How to handle contracts deployed by or related to this account
    pub child_contract_scope: ChildContractScope,
}

/// Defines how child contracts (proxies, factories, clones) are handled in the Safe Harbor scope
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum ChildContractScope {
    /// Only this exact contract, no child contracts included
    None,

    /// This contract + child contracts that existed before the adoption date
    ExistingOnly,

    /// This contract + all child contracts (past and future)
    All,

    /// This contract + only child contracts deployed after the adoption date
    FutureOnly,
}

impl Default for ChildContractScope {
    fn default() -> Self {
        Self::None
    }
}

/// Identity verification level required from the whitehat security researcher
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum IdentityRequirements {
    /// No identity verification required, fully anonymous
    Anonymous,

    /// Some persistent identifier required (e.g., GitHub handle, ENS name)
    Pseudonymous,

    /// Full legal identity required
    Named,
}

impl Default for IdentityRequirements {
    fn default() -> Self {
        Self::Anonymous
    }
}
/// Bounty terms defining the rewards and requirements for whitehat security researchers
/// who recover assets during an active exploit under the Safe Harbor agreement.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct BountyTerms {
    /// Percentage of recovered assets awarded to the whitehat (0-100)
    /// Example: 10 = 10% of recovered funds
    pub bounty_percentage: u64,

    /// Maximum bounty cap in USD for a single incident
    /// Example: 100000 = $100,000 max bounty per exploit
    /// Set to 0 for no per-incident cap
    pub bounty_cap_usd: u64,

    /// Whether the whitehat can retain assets beyond the bounty cap
    /// If true, whitehat keeps everything above the aggregate_bounty_cap_usd
    /// Cannot be true if aggregate_bounty_cap_usd is set (validation enforced)
    pub retainable: bool,

    /// Identity verification requirements for the whitehat
    /// Determines level of anonymity allowed (Anonymous, Pseudonymous, Named)
    pub identity: IdentityRequirements,

    /// Free-form text describing diligence requirements
    /// Example: "Responsible disclosure required", "Must provide detailed report"
    pub diligence_requirements: String,

    /// Total aggregate cap across all incidents in USD
    /// Example: 500000 = $500,000 total max across all bounties
    /// Set to 0 for no aggregate cap
    /// Cannot be set if retainable is true (validation enforced)
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
            account_address: "0x123".to_string(),
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
