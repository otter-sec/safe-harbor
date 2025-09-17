use crate::errors::{AdoptionError, ValidationError};
use anchor_lang::prelude::*;

#[account]
pub struct Adopt {
    pub agreement: Pubkey,
}

impl Adopt {
    pub const SPACE: usize = 8 + 32;

    pub fn validate_initialized(&self) -> Result<()> {
        require!(
            self.agreement != Pubkey::default(),
            AdoptionError::AdoptionEntryNotFound
        );
        Ok(())
    }

    pub fn validate_agreement(&self) -> Result<()> {
        require!(
            self.agreement != Pubkey::default(),
            ValidationError::InvalidAccountParams
        );
        Ok(())
    }

    pub fn validate_can_update(&self, new_agreement: &Pubkey) -> Result<()> {
        require!(
            new_agreement != &Pubkey::default(),
            ValidationError::InvalidAccountParams
        );
        Ok(())
    }
}
