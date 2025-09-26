use crate::errors::*;
use anchor_lang::prelude::*;

pub const MAX_REGISTRY_CHAINS: usize = 256;

/// Registry account for managing Safe Harbor agreements
#[account]
#[derive(Default, InitSpace)]
pub struct Registry {
    pub owner: Pubkey,
    pub initialized: bool,
    #[max_len(MAX_REGISTRY_CHAINS)]
    pub valid_chains: Vec<u64>,
}

impl Registry {
    pub const REGISTRY_SEED: &'static [u8] = b"registry_v2";

    pub fn validate_owner(&self, caller: &Pubkey) -> Result<()> {
        require!(self.initialized, RegistryError::RegistryNotInitialized);
        require!(self.owner == *caller, RegistryError::NotRegistryOwner);
        Ok(())
    }

    pub fn validate_chain_id(&self, chain_id: u64) -> Result<()> {
        if chain_id == 0 {
            return Err(ChainError::InvalidChainId.into());
        }

        if !self.valid_chains.contains(&chain_id) {
            return Err(ChainError::ChainNotValidInRegistry.into());
        }

        Ok(())
    }

    pub fn add_valid_chains(&mut self, chain_ids: Vec<u64>) -> Result<()> {
        for chain_id in chain_ids {
            if !self.valid_chains.contains(&chain_id) {
                self.valid_chains.push(chain_id);
            }
        }

        Ok(())
    }

    pub fn remove_invalid_chains(&mut self, chain_ids: Vec<u64>) -> Result<()> {
        self.valid_chains
            .retain(|chain_id| !chain_ids.contains(chain_id));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Test the default values of the registry
    fn test_registry_default() {
        let registry = Registry::default();
        assert!(!registry.initialized);
        assert_eq!(registry.owner, Pubkey::default());
        assert!(registry.valid_chains.is_empty());
    }

    #[test]
    /// Test the validate_chain_id function with valid and invalid chain ids
    fn test_validate_chain_id() {
        let mut registry = Registry::default();
        registry.add_valid_chains(vec![1]).unwrap();
        assert!(registry.validate_chain_id(0).is_err());
        assert!(registry.validate_chain_id(1).is_ok());
    }

    #[test]
    /// Test the add_valid_chains function and remove_invalid_chains function
    fn test_add_valid_chains() {
        let mut registry = Registry::default();
        registry.add_valid_chains(vec![1]).unwrap();
        assert!(registry.valid_chains.contains(&1));
        registry.remove_invalid_chains(vec![1]).unwrap();
        assert!(!registry.valid_chains.contains(&1));
    }
}
