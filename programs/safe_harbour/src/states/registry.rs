use crate::errors::ValidationError;
use anchor_lang::prelude::*;

pub const MAX_AGREEMENTS: usize = 64;
pub const MAX_CHAINS: usize = 32;
pub const MAX_CHAIN_NAME_LEN: usize = 64;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]

pub struct AccountMapEntry {
    pub key: Pubkey,
    pub value: Pubkey,
}

/// Registry account for managing Safe Harbor agreements
#[account]
#[derive(Default)]
pub struct Registry {
    pub owner: Pubkey,
    pub initialized: bool,
    pub agreements: Vec<AccountMapEntry>,
    pub valid_chains: Vec<String>,
}

impl Registry {
    pub const INITIAL_SPACE: usize =
        8 + 32 + 1 + 4 + (MAX_AGREEMENTS * 64) + 4 + (MAX_CHAINS * (4 + MAX_CHAIN_NAME_LEN)) + 1024;

    pub fn get_agreement(&self, adopter: Pubkey) -> Option<Pubkey> {
        self.agreements
            .iter()
            .find(|e| e.key == adopter)
            .map(|e| e.value)
    }

    pub fn set_agreement(&mut self, adopter: Pubkey, agreement: Pubkey) -> Result<()> {
        require!(
            self.agreements.len() + 1 <= MAX_AGREEMENTS,
            crate::errors::ValidationError::MaxLengthExceeded
        );
        if let Some(i) = self.agreements.iter().position(|e| e.key == adopter) {
            self.agreements[i].value = agreement;
        } else {
            self.agreements.push(AccountMapEntry {
                key: adopter,
                value: agreement,
            });
        }
        Ok(())
    }

    pub fn remove_agreement(&mut self, adopter: Pubkey) -> bool {
        if let Some(i) = self.agreements.iter().position(|e| e.key == adopter) {
            self.agreements.remove(i);
            true
        } else {
            false
        }
    }

    pub fn validate_owner(&self, caller: &Pubkey) -> Result<()> {
        require!(
            self.owner == *caller,
            crate::errors::RegistryError::NotRegistryOwner
        );
        Ok(())
    }

    pub fn validate_chain_id(&self, chain_id: &str) -> Result<()> {
        if chain_id.is_empty() {
            return Err(ValidationError::InvalidChainParams.into());
        }

        if !self.is_chain_valid(chain_id) {
            return Err(crate::errors::ChainError::ChainNotValidInRegistry.into());
        }

        Ok(())
    }

    pub fn is_chain_valid(&self, chain_id: &str) -> bool {
        self.valid_chains.contains(&chain_id.to_string())
    }

    pub fn add_valid_chains(&mut self, chain_ids: Vec<String>) -> Result<()> {
        require!(
            self.valid_chains.len() + chain_ids.len() <= MAX_CHAINS,
            crate::errors::ValidationError::MaxLengthExceeded
        );
        for chain_id in chain_ids {
            if !self.valid_chains.contains(&chain_id) {
                require!(
                    chain_id.len() <= MAX_CHAIN_NAME_LEN,
                    crate::errors::ValidationError::InvalidChainParams
                );

                self.valid_chains.push(chain_id);
            }
        }

        Ok(())
    }

    pub fn remove_invalid_chains(&mut self, chain_ids: Vec<String>) -> Result<()> {
        self.valid_chains
            .retain(|chain_id| !chain_ids.contains(chain_id));
        Ok(())
    }
}
