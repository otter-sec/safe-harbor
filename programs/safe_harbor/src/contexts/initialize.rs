use anchor_lang::prelude::*;

use crate::errors::*;
use crate::states::registry::Registry;
use crate::utils::events::{RegistryInitialized, ValidChainsSet};

/// Initializes the global Registry PDA.
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_if_needed,
        seeds = [Registry::REGISTRY_SEED],
        bump,
        payer = signer,
        space = 8 + Registry::INIT_SPACE,
    )]
    pub registry: Account<'info, Registry>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, valid_chains: Vec<u64>) -> Result<()> {
        require!(
            !valid_chains.is_empty(),
            crate::errors::ChainError::InvalidChainId
        );

        let registry = &mut self.registry;
        require!(!registry.initialized, RegistryError::AlreadyInitialized);

        registry.initialized = true;
        registry.owner = self.signer.key();

        registry.add_valid_chains(valid_chains)?;

        emit!(RegistryInitialized {
            owner: self.signer.key(),
            registry: self.registry.key(),
            valid_chains: self.registry.valid_chains.clone()
        });

        Ok(())
    }

    pub fn set_valid_chains(&mut self, valid_chains: Vec<u64>) -> Result<()> {
        self.registry.validate_owner(&self.signer.key())?;
        self.registry.add_valid_chains(valid_chains)?;

        emit!(ValidChainsSet {
            valid_chains: self.registry.valid_chains.clone()
        });
        Ok(())
    }

    pub fn set_invalid_chains(&mut self, invalid_chains: Vec<u64>) -> Result<()> {
        self.registry.validate_owner(&self.signer.key())?;
        self.registry.remove_invalid_chains(invalid_chains)?;

        emit!(ValidChainsSet {
            valid_chains: self.registry.valid_chains.clone()
        });
        Ok(())
    }
}
