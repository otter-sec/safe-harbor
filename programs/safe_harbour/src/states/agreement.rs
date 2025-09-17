use crate::errors::{BountyError, ChainError, ValidationError};
use crate::utils::types::{AccountInScope, BountyTerms, Chain, Contact};
use anchor_lang::prelude::*;

#[account]
pub struct Agreement {
    pub owner: Pubkey,
    pub data: crate::utils::types::AgreementData,
}

impl Agreement {
    pub const BASE_SPACE: usize = 8 + 32 + 4 + 4 + 4 + 4 + 8 + 8 + 1 + 1 + 4 + 8 + 4; // agreement_uri string length

    pub const INITIAL_SPACE: usize = Self::BASE_SPACE + 1024;

    pub fn calculate_required_space(&self) -> usize {
        let mut size = Self::BASE_SPACE;

        size += self.data.protocol_name.len();

        for contact in &self.data.contact_details {
            size += 4 + contact.name.len() + 4 + contact.contact.len();
        }

        for chain in &self.data.chains {
            size += 4 + chain.asset_recovery_address.len();
            size += 4;

            for account in &chain.accounts {
                size += 4 + account.account_address.len() + 1; // string + enum
            }

            size += 4 + chain.caip2_chain_id.len();
        }

        size += self.data.bounty_terms.diligence_requirements.len();

        size += self.data.agreement_uri.len();
        size + (size / 4)
    }

    pub fn find_chain_index(&self, caip2_chain_id: &str) -> Option<usize> {
        self.data
            .chains
            .iter()
            .position(|chain| chain.caip2_chain_id == caip2_chain_id)
    }

    pub fn find_account_index(&self, chain_index: usize, account_address: &str) -> Option<usize> {
        if chain_index < self.data.chains.len() {
            self.data.chains[chain_index]
                .accounts
                .iter()
                .position(|account| account.account_address == account_address)
        } else {
            None
        }
    }

    pub fn validate_bounty_terms(&self) -> bool {
        !(self.data.bounty_terms.aggregate_bounty_cap_usd > 0 && self.data.bounty_terms.retainable)
    }

    /// Validates the entire agreement data
    pub fn validate_agreement_data(&self, registry: &crate::states::Registry) -> Result<()> {
        Self::validate_protocol_name(&self.data.protocol_name)?;
        Self::validate_contact_details(&self.data.contact_details)?;
        Self::validate_chains(&self.data.chains, registry)?;
        Self::validate_bounty_terms_data(&self.data.bounty_terms)?;
        Self::validate_agreement_uri(&self.data.agreement_uri)?;
        Ok(())
    }

    /// Validates protocol name
    pub fn validate_protocol_name(protocol_name: &str) -> Result<()> {
        if protocol_name.is_empty() {
            return Err(ValidationError::InvalidProtocolName.into());
        }

        if protocol_name.len() > 100 {
            return Err(ValidationError::InvalidProtocolName.into());
        }

        if protocol_name.trim().is_empty() {
            return Err(ValidationError::InvalidProtocolName.into());
        }

        Ok(())
    }

    /// Validates contact details
    pub fn validate_contact_details(contact_details: &[Contact]) -> Result<()> {
        if contact_details.is_empty() {
            return Err(ValidationError::InvalidContactDetails.into());
        }

        if contact_details.len() > 10 {
            return Err(ValidationError::InvalidContactDetails.into());
        }

        for contact in contact_details {
            if contact.name.is_empty() {
                return Err(ValidationError::InvalidContactDetails.into());
            }

            if contact.contact.is_empty() {
                return Err(ValidationError::InvalidContactDetails.into());
            }

            if contact.name.len() > 50 {
                return Err(ValidationError::InvalidContactDetails.into());
            }

            if contact.contact.len() > 100 {
                return Err(ValidationError::InvalidContactDetails.into());
            }
        }

        Ok(())
    }

    /// Validates chains
    pub fn validate_chains(chains: &[Chain], registry: &crate::states::Registry) -> Result<()> {
        if chains.is_empty() {
            return Err(ValidationError::InvalidChainParams.into());
        }

        if chains.len() > 50 {
            return Err(ValidationError::InvalidChainParams.into());
        }

        let mut seen_chain_ids = std::collections::HashSet::new();

        for chain in chains {
            Self::validate_caip2_chain_id(&chain.caip2_chain_id)?;

            if !seen_chain_ids.insert(&chain.caip2_chain_id) {
                return Err(ChainError::DuplicateChainId.into());
            }

            require!(
                registry.is_chain_valid(&chain.caip2_chain_id),
                ChainError::ChainNotValidInRegistry
            );

            Self::validate_account_address(&chain.asset_recovery_address, &chain.caip2_chain_id)?;
            Self::validate_accounts(&chain.accounts, &chain.caip2_chain_id)?;
        }

        Ok(())
    }

    /// Validates CAIP-2 chain ID
    pub fn validate_caip2_chain_id(caip2_chain_id: &str) -> Result<()> {
        if caip2_chain_id.is_empty() {
            return Err(ChainError::InvalidChainId.into());
        }

        if !caip2_chain_id.contains(':') {
            return Err(ChainError::InvalidChainId.into());
        }

        let parts: Vec<&str> = caip2_chain_id.split(':').collect();
        if parts.len() != 2 {
            return Err(ChainError::InvalidChainId.into());
        }

        let namespace = parts[0];
        let reference = parts[1];

        if namespace.is_empty() {
            return Err(ChainError::InvalidChainId.into());
        }

        if reference.is_empty() {
            return Err(ChainError::InvalidChainId.into());
        }

        if namespace == "eip155" {
            if reference.parse::<u64>().is_err() {
                return Err(ChainError::InvalidChainId.into());
            }
        }

        Ok(())
    }

    /// Validates account address
    pub fn validate_account_address(account_address: &str, caip2_chain_id: &str) -> Result<()> {
        if account_address.is_empty() {
            return Err(ValidationError::InvalidAccountParams.into());
        }
        let namespace = caip2_chain_id.split(':').next().unwrap_or("");

        match namespace {
            "eip155" => {
                if !account_address.starts_with("0x") || account_address.len() != 42 {
                    return Err(ValidationError::InvalidAccountParams.into());
                }

                let hex_part = &account_address[2..];
                if !hex_part.chars().all(|c| c.is_ascii_hexdigit()) {
                    return Err(ValidationError::InvalidAccountParams.into());
                }
            }
            "solana" => {
                if account_address.len() < 32 || account_address.len() > 44 {
                    return Err(ValidationError::InvalidAccountParams.into());
                }

                if !account_address
                    .chars()
                    .all(|c| c.is_alphanumeric() && c != '0' && c != 'O' && c != 'I' && c != 'l')
                {
                    return Err(ValidationError::InvalidAccountParams.into());
                }
            }
            _ => {}
        }

        Ok(())
    }

    /// Validates accounts in scope
    pub fn validate_accounts(accounts: &[AccountInScope], caip2_chain_id: &str) -> Result<()> {
        if accounts.is_empty() {
            return Err(ValidationError::InvalidAccountParams.into());
        }

        if accounts.len() > 100 {
            return Err(ValidationError::InvalidAccountParams.into());
        }

        let mut seen_addresses = std::collections::HashSet::new();

        for account in accounts {
            Self::validate_account_address(&account.account_address, caip2_chain_id)?;

            if !seen_addresses.insert(&account.account_address) {
                return Err(ValidationError::InvalidAccountParams.into());
            }
        }

        Ok(())
    }

    /// Validates bounty terms data
    pub fn validate_bounty_terms_data(bounty_terms: &BountyTerms) -> Result<()> {
        if bounty_terms.bounty_percentage > 100 {
            return Err(BountyError::InvalidBountyPercentage.into());
        }

        if bounty_terms.bounty_cap_usd == 0 {
            return Err(BountyError::InvalidBountyCapUsd.into());
        }

        if bounty_terms.aggregate_bounty_cap_usd > 0
            && bounty_terms.aggregate_bounty_cap_usd < bounty_terms.bounty_cap_usd
        {
            return Err(BountyError::InvalidAggregateBountyCapUsd.into());
        }

        if bounty_terms.aggregate_bounty_cap_usd > 0 && bounty_terms.retainable {
            return Err(BountyError::CannotSetBothAggregateBountyCapUSDAndRetainable.into());
        }

        Ok(())
    }

    /// Validates agreement URI
    pub fn validate_agreement_uri(agreement_uri: &str) -> Result<()> {
        if agreement_uri.is_empty() {
            return Err(ValidationError::InvalidAgreementUri.into());
        }

        if agreement_uri.len() > 200 {
            return Err(ValidationError::InvalidAgreementUri.into());
        }

        let valid_protocols = ["ipfs://", "https://", "http://", "ar://"];
        if !valid_protocols
            .iter()
            .any(|protocol| agreement_uri.starts_with(protocol))
        {
            return Err(ValidationError::InvalidAgreementUri.into());
        }

        Ok(())
    }
}
