use anchor_lang::prelude::*;

use crate::constants::REGISTRY_SEED;
use crate::errors::*;
use crate::states::Registry;
use crate::utils::events::{RegistryInitialized, ValidChainsSet};

/// Initializes the global Registry PDA.
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_if_needed,
        seeds = [REGISTRY_SEED],
        bump,
        payer = signer,
        space = Registry::INITIAL_SPACE,
    )]
    pub registry: Account<'info, Registry>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, valid_chains: Vec<String>) -> Result<()> {
        require!(
            !valid_chains.is_empty(),
            crate::errors::ValidationError::InvalidChainParams
        );
        let registry = &mut self.registry;
        require!(!registry.initialized, ValidationError::AlreadyInitialized);

        registry.initialized = true;
        registry.owner = self.signer.key();
        
        registry.add_valid_chains(valid_chains)?;

        emit!(RegistryInitialized {
            owner: self.signer.key(),
            registry: self.registry.key()
        });

        Ok(())
    }

    pub fn set_valid_chains(&mut self, valid_chains: Vec<String>) -> Result<()> {
        self.registry.validate_owner(&self.signer.key())?;
        self.registry.add_valid_chains(valid_chains)?;

        emit!(ValidChainsSet {
            valid_chains: self.registry.valid_chains.clone()
        });
        Ok(())
    }

    pub fn set_invalid_chains(&mut self, invalid_chains: Vec<String>) -> Result<()> {
        self.registry.validate_owner(&self.signer.key())?;
        self.registry.remove_invalid_chains(invalid_chains)?;

        emit!(ValidChainsSet {
            valid_chains: self.registry.valid_chains.clone()
        });
        Ok(())
    }
}
