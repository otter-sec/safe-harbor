use super::account_map::AccountMap;
use crate::errors::ValidationError;
use anchor_lang::prelude::*;

/// Registry account for managing Safe Harbor agreements
#[account]
pub struct Registry {
    pub owner: Pubkey,
    pub agreements: AccountMap,
    pub valid_chains: Vec<String>,
}

impl Registry {
    pub const BASE_SPACE: usize = 8 + 32 + 4 + 4;

    pub const INITIAL_SPACE: usize = Self::BASE_SPACE + 1024;

    pub fn calculate_required_space(&self) -> usize {
        let mut size = Self::BASE_SPACE;
        size += self.agreements.items.len() * (32 + 32); // key + value pubkeys

        for chain in &self.valid_chains {
            size += 4 + chain.len(); // string length + string data
        }
        size + (size / 4)
    }

    pub fn is_chain_valid(&self, chain_id: &str) -> bool {
        self.valid_chains.contains(&chain_id.to_string())
    }

    pub fn add_valid_chain(&mut self, chain_id: String) {
        if !self.valid_chains.contains(&chain_id) {
            self.valid_chains.push(chain_id);
        }
    }

    pub fn remove_valid_chain(&mut self, chain_id: &str) -> bool {
        if let Some(pos) = self.valid_chains.iter().position(|x| x == chain_id) {
            self.valid_chains.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn get_agreement(&self, adopter: Pubkey) -> Option<Pubkey> {
        self.agreements.get(adopter)
    }

    pub fn set_agreement(&mut self, adopter: Pubkey, agreement: Pubkey) {
        self.agreements.insert(adopter, agreement);
    }

    /// Validates that the caller is the registry owner
    pub fn validate_owner(&self, caller: &Pubkey) -> Result<()> {
        require!(
            self.owner == *caller,
            crate::errors::RegistryError::NotRegistryOwner
        );
        Ok(())
    }

    /// Validates that a chain ID is valid
    pub fn validate_chain_id(&self, chain_id: &str) -> Result<()> {
        if chain_id.is_empty() {
            return Err(ValidationError::InvalidChainParams.into());
        }

        if !self.is_chain_valid(chain_id) {
            return Err(crate::errors::ChainError::ChainNotValidInRegistry.into());
        }

        Ok(())
    }

    /// Validates that the registry is properly initialized
    pub fn validate_initialized(&self) -> Result<()> {
        require!(
            self.owner != Pubkey::default(),
            crate::errors::RegistryError::RegistryNotInitialized
        );
        Ok(())
    }

    /// Validates that an adopter exists in the registry
    pub fn validate_adopter_exists(&self, adopter: &Pubkey) -> Result<()> {
        require!(
            self.agreements.contains_key(*adopter),
            crate::errors::AdoptionError::NoAgreement
        );
        Ok(())
    }

    /// Validates that an adopter doesn't already exist in the registry
    pub fn validate_adopter_not_exists(&self, adopter: &Pubkey) -> Result<()> {
        require!(
            !self.agreements.contains_key(*adopter),
            crate::errors::AdoptionError::AdoptionAlreadyExists
        );
        Ok(())
    }
}
