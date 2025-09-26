use anchor_lang::prelude::*;

#[error_code]
pub enum RegistryError {
    #[msg("Registry has not been initialized")]
    RegistryNotInitialized,

    #[msg("Registry already initialized")]
    AlreadyInitialized,

    #[msg("Caller is not the registry owner")]
    NotRegistryOwner,
}

#[error_code]
pub enum ChainError {
    #[msg("Invalid CAIP-2 chain ID")]
    InvalidChainId,

    #[msg("Chain not valid in registry")]
    ChainNotValidInRegistry,
}

#[error_code]
pub enum ValidationError {
    #[msg("Invalid update type")]
    InvalidUpdateType,

    #[msg("Invalid protocol name")]
    InvalidProtocolName,

    #[msg("Invalid contact details")]
    InvalidContactDetails,

    #[msg("Invalid agreement URI")]
    InvalidAgreementUri,

    #[msg("Invalid bounty percentage. Must be between 0 and 100")]
    InvalidBountyPercentage,

    #[msg("Invalid bounty cap in USD")]
    InvalidBountyCapUsd,

    #[msg("Invalid aggregate bounty cap in USD")]
    InvalidAggregateBountyCapUsd,
}
