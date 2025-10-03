use anchor_lang::prelude::*;

use crate::states::{AdoptUpdateType, AgreementUpdateType};

#[event]
pub struct SafeHarborAdopted {
    pub adopter: Pubkey,
    pub chain_id: String,
    pub agreement: Pubkey,
    pub asset_recovery_address: String,
}

#[event]
pub struct AgreementUpdated {
    pub agreement: Pubkey,
    pub update_type: AgreementUpdateType,
}

#[event]
pub struct AdoptionUpdated {
    pub adopter: Pubkey,
    pub chain_id: String,
    pub adoption: Pubkey,
    pub update_type: AdoptUpdateType,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_type_enums() {
        // Test agreement update types
        let update_type = AgreementUpdateType::ProtocolName;

        let serialized = update_type.try_to_vec().unwrap();
        let deserialized: AgreementUpdateType =
            AgreementUpdateType::try_from_slice(&serialized).unwrap();
        let original_serialized = update_type.try_to_vec().unwrap();
        let deserialized_serialized = deserialized.try_to_vec().unwrap();
        assert_eq!(original_serialized, deserialized_serialized);
    }
}
