use anchor_lang::prelude::*;

use crate::errors::{BountyError, ChainError, ValidationError};
use crate::utils::types::{AccountInScope, BountyTerms, Chain};
use crate::utils::types::Contact;

pub fn validate_bounty_terms(bounty_terms: &BountyTerms) -> Result<()> {
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

pub fn validate_chains(chains: &[Chain]) -> Result<()> {
    if chains.is_empty() {
        return Err(ValidationError::InvalidChainParams.into());
    }

    if chains.len() > 50 {
        return Err(ValidationError::InvalidChainParams.into());
    }

    let mut seen_chain_ids = std::collections::HashSet::new();

    for chain in chains {
        validate_caip2_chain_id(&chain.caip2_chain_id)?;

        if !seen_chain_ids.insert(&chain.caip2_chain_id) {
            return Err(ChainError::DuplicateChainId.into());
        }

        validate_account_address(&chain.asset_recovery_address, &chain.caip2_chain_id)?;

        validate_accounts(&chain.accounts, &chain.caip2_chain_id)?;
    }

    Ok(())
}

pub fn validate_accounts(accounts: &[AccountInScope], caip2_chain_id: &str) -> Result<()> {
    if accounts.is_empty() {
        return Err(ValidationError::InvalidAccountParams.into());
    }

    if accounts.len() > 100 {
        return Err(ValidationError::InvalidAccountParams.into());
    }

    let mut seen_addresses = std::collections::HashSet::new();

    for account in accounts {
        validate_account_address(&account.account_address, caip2_chain_id)?;

        if !seen_addresses.insert(&account.account_address) {
            return Err(ValidationError::InvalidAccountParams.into());
        }
    }

    Ok(())
}

pub fn calculate_required_space(data_size: usize) -> usize {
    8 + data_size + (data_size / 4)
}

pub fn find_adoption_pda(
    adopter: &Pubkey,
    agreement: &Pubkey,
    program_id: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"adoption_v2", adopter.as_ref(), agreement.as_ref()],
        program_id,
    )
}

pub fn find_adoption_head_pda(adopter: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"adoption_head", adopter.as_ref()], program_id)
}

pub fn find_registry_pda(program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"registry_v2"], program_id)
}
