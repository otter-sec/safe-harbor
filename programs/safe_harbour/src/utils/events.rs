use anchor_lang::prelude::*;

#[event]
pub struct RegistryInitialized {
    pub registry: Pubkey,
    pub owner: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct ChainValidityChanged {
    pub caip2_chain_id: String,
    pub is_valid: bool,
    pub registry: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct SafeHarborAdopted {
    pub adopter: Pubkey,
    pub old_agreement: Pubkey,
    pub new_agreement: Pubkey,
    pub registry: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct AgreementCreated {
    pub agreement: Pubkey,
    pub owner: Pubkey,
    pub protocol_name: String,
    pub timestamp: i64,
}

#[event]
pub struct AgreementUpdated {
    pub agreement: Pubkey,
    pub owner: Pubkey,
    pub update_type: AgreementUpdateType,
    pub timestamp: i64,
}

#[event]
pub struct AgreementOwnershipTransferred {
    pub agreement: Pubkey,
    pub old_owner: Pubkey,
    pub new_owner: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct ProtocolNameUpdated {
    pub agreement: Pubkey,
    pub owner: Pubkey,
    pub old_name: String,
    pub new_name: String,
    pub timestamp: i64,
}

#[event]
pub struct ContactDetailsUpdated {
    pub agreement: Pubkey,
    pub owner: Pubkey,
    pub contact_count: u32,
    pub timestamp: i64,
}

#[event]
pub struct ChainsUpdated {
    pub agreement: Pubkey,
    pub owner: Pubkey,
    pub update_type: ChainUpdateType,
    pub chain_count: u32,
    pub timestamp: i64,
}

#[event]
pub struct AccountsUpdated {
    pub agreement: Pubkey,
    pub owner: Pubkey,
    pub caip2_chain_id: String,
    pub update_type: AccountUpdateType,
    pub account_count: u32,
    pub timestamp: i64,
}

#[event]
pub struct BountyTermsUpdated {
    pub agreement: Pubkey,
    pub owner: Pubkey,
    pub bounty_percentage: u64,
    pub bounty_cap_usd: u64,
    pub retainable: bool,
    pub timestamp: i64,
}

#[event]
pub struct AgreementUriUpdated {
    pub agreement: Pubkey,
    pub owner: Pubkey,
    pub old_uri: String,
    pub new_uri: String,
    pub timestamp: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum AgreementUpdateType {
    ProtocolName,
    ContactDetails,
    Chains,
    Accounts,
    BountyTerms,
    AgreementUri,
    Ownership,
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

impl RegistryInitialized {
    pub fn new(registry: Pubkey, owner: Pubkey) -> Self {
        Self {
            registry,
            owner,
            timestamp: Clock::get().unwrap().unix_timestamp,
        }
    }
}

impl ChainValidityChanged {
    pub fn new(caip2_chain_id: String, is_valid: bool, registry: Pubkey) -> Self {
        Self {
            caip2_chain_id,
            is_valid,
            registry,
            timestamp: Clock::get().unwrap().unix_timestamp,
        }
    }
}

impl SafeHarborAdopted {
    pub fn new(adopter: Pubkey, old_agreement: Pubkey, new_agreement: Pubkey, registry: Pubkey) -> Self {
        Self {
            adopter,
            old_agreement,
            new_agreement,
            registry,
            timestamp: Clock::get().unwrap().unix_timestamp,
        }
    }
}

impl AgreementCreated {
    pub fn new(agreement: Pubkey, owner: Pubkey, protocol_name: String) -> Self {
        Self {
            agreement,
            owner,
            protocol_name,
            timestamp: Clock::get().unwrap().unix_timestamp,
        }
    }
}

impl AgreementUpdated {
    pub fn new(agreement: Pubkey, owner: Pubkey, update_type: AgreementUpdateType) -> Self {
        Self {
            agreement,
            owner,
            update_type,
            timestamp: Clock::get().unwrap().unix_timestamp,
        }
    }
}

impl AgreementOwnershipTransferred {
    pub fn new(agreement: Pubkey, old_owner: Pubkey, new_owner: Pubkey) -> Self {
        Self {
            agreement,
            old_owner,
            new_owner,
            timestamp: Clock::get().unwrap().unix_timestamp,
        }
    }
}

impl ProtocolNameUpdated {
    pub fn new(agreement: Pubkey, owner: Pubkey, old_name: String, new_name: String) -> Self {
        Self {
            agreement,
            owner,
            old_name,
            new_name,
            timestamp: Clock::get().unwrap().unix_timestamp,
        }
    }
}

impl ContactDetailsUpdated {
    pub fn new(agreement: Pubkey, owner: Pubkey, contact_count: u32) -> Self {
        Self {
            agreement,
            owner,
            contact_count,
            timestamp: Clock::get().unwrap().unix_timestamp,
        }
    }
}

impl ChainsUpdated {
    pub fn new(agreement: Pubkey, owner: Pubkey, update_type: ChainUpdateType, chain_count: u32) -> Self {
        Self {
            agreement,
            owner,
            update_type,
            chain_count,
            timestamp: Clock::get().unwrap().unix_timestamp,
        }
    }
}

impl AccountsUpdated {
    pub fn new(agreement: Pubkey, owner: Pubkey, caip2_chain_id: String, update_type: AccountUpdateType, account_count: u32) -> Self {
        Self {
            agreement,
            owner,
            caip2_chain_id,
            update_type,
            account_count,
            timestamp: Clock::get().unwrap().unix_timestamp,
        }
    }
}

impl BountyTermsUpdated {
    pub fn new(agreement: Pubkey, owner: Pubkey, bounty_percentage: u64, bounty_cap_usd: u64, retainable: bool) -> Self {
        Self {
            agreement,
            owner,
            bounty_percentage,
            bounty_cap_usd,
            retainable,
            timestamp: Clock::get().unwrap().unix_timestamp,
        }
    }
}

impl AgreementUriUpdated {
    pub fn new(agreement: Pubkey, owner: Pubkey, old_uri: String, new_uri: String) -> Self {
        Self {
            agreement,
            owner,
            old_uri,
            new_uri,
            timestamp: Clock::get().unwrap().unix_timestamp,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_creation() {
        // Test that events can be created with proper timestamps
        let registry = Pubkey::new_unique();
        let owner = Pubkey::new_unique();
        let adopter = Pubkey::new_unique();
        let agreement = Pubkey::new_unique();

        // Test registry initialized event
        let registry_event = RegistryInitialized::new(registry, owner);
        assert_eq!(registry_event.registry, registry);
        assert_eq!(registry_event.owner, owner);

        // Test chain validity changed event
        let chain_event = ChainValidityChanged::new("eip155:1".to_string(), true, registry);
        assert_eq!(chain_event.caip2_chain_id, "eip155:1");
        assert!(chain_event.is_valid);

        // Test safe harbor adopted event
        let adoption_event = SafeHarborAdopted::new(adopter, Pubkey::default(), agreement, registry);
        assert_eq!(adoption_event.adopter, adopter);
        assert_eq!(adoption_event.new_agreement, agreement);

        // Test agreement created event
        let agreement_event = AgreementCreated::new(agreement, owner, "Test Protocol".to_string());
        assert_eq!(agreement_event.agreement, agreement);
        assert_eq!(agreement_event.protocol_name, "Test Protocol");

        // Test agreement updated event
        let update_event = AgreementUpdated::new(agreement, owner, AgreementUpdateType::ProtocolName);
        assert_eq!(update_event.agreement, agreement);
        match update_event.update_type {
            AgreementUpdateType::ProtocolName => {},
            _ => panic!("Update type should be ProtocolName"),
        }

        // Test ownership transferred event
        let transfer_event = AgreementOwnershipTransferred::new(agreement, owner, adopter);
        assert_eq!(transfer_event.agreement, agreement);
        assert_eq!(transfer_event.old_owner, owner);
        assert_eq!(transfer_event.new_owner, adopter);
    }

    #[test]
    fn test_update_type_enums() {
        // Test agreement update types
        let update_types = vec![
            AgreementUpdateType::ProtocolName,
            AgreementUpdateType::ContactDetails,
            AgreementUpdateType::Chains,
            AgreementUpdateType::Accounts,
            AgreementUpdateType::BountyTerms,
            AgreementUpdateType::AgreementUri,
            AgreementUpdateType::Ownership,
        ];

        for update_type in update_types {
            // Test serialization
            let serialized = update_type.try_to_vec().unwrap();
            let deserialized: AgreementUpdateType = AgreementUpdateType::try_from_slice(&serialized).unwrap();
                // Compare by serializing both and checking equality
            let original_serialized = update_type.try_to_vec().unwrap();
            let deserialized_serialized = deserialized.try_to_vec().unwrap();
            assert_eq!(original_serialized, deserialized_serialized);
        }

        // Test chain update types
        let chain_update_types = vec![
            ChainUpdateType::Added,
            ChainUpdateType::Updated,
            ChainUpdateType::Removed,
        ];

        for update_type in chain_update_types {
            let serialized = update_type.try_to_vec().unwrap();
            let deserialized: ChainUpdateType = ChainUpdateType::try_from_slice(&serialized).unwrap();
                let original_serialized = update_type.try_to_vec().unwrap();
            let deserialized_serialized = deserialized.try_to_vec().unwrap();
            assert_eq!(original_serialized, deserialized_serialized);
        }

        // Test account update types
        let account_update_types = vec![
            AccountUpdateType::Added,
            AccountUpdateType::Removed,
        ];

        for update_type in account_update_types {
            let serialized = update_type.try_to_vec().unwrap();
            let deserialized: AccountUpdateType = AccountUpdateType::try_from_slice(&serialized).unwrap();
                let original_serialized = update_type.try_to_vec().unwrap();
            let deserialized_serialized = deserialized.try_to_vec().unwrap();
            assert_eq!(original_serialized, deserialized_serialized);
        }
    }

    #[test]
    fn test_event_serialization() {
        // Test that events can be serialized and deserialized
        let owner = Pubkey::new_unique();
        let agreement = Pubkey::new_unique();

        let event = AgreementCreated {
            agreement,
            owner,
            protocol_name: "Test Protocol".to_string(),
            timestamp: 1234567890,
        };

        // Test serialization
        let serialized = event.try_to_vec().unwrap();
        let deserialized: AgreementCreated = AgreementCreated::try_from_slice(&serialized).unwrap();

        assert_eq!(event.agreement, deserialized.agreement);
        assert_eq!(event.owner, deserialized.owner);
        assert_eq!(event.protocol_name, deserialized.protocol_name);
        assert_eq!(event.timestamp, deserialized.timestamp);
    }
}