use anchor_lang::prelude::*;
#[error_code]
pub enum RegistryError {
    #[msg("Registry has not been initialized")]
    RegistryNotInitialized,

    #[msg("Caller is not the registry owner")]
    NotRegistryOwner,
}
#[error_code]
pub enum ChainError {
    #[msg("Invalid CAIP-2 chain ID")]
    InvalidChainId,

    #[msg("Max length exceeded")]
    MaxLengthExceeded,

    #[msg("Duplicate CAIP-2 chain ID")]
    DuplicateChainId,

    #[msg("Chain not valid in registry")]
    ChainNotValidInRegistry,
}
#[error_code]
pub enum BountyError {
    #[msg("Invalid bounty percentage. Must be between 0 and 100")]
    InvalidBountyPercentage,

    #[msg("Invalid bounty cap in USD")]
    InvalidBountyCapUsd,

    #[msg("Invalid aggregate bounty cap in USD")]
    InvalidAggregateBountyCapUsd,

    #[msg("Cannot set both aggregate bounty cap USD and retainable")]
    CannotSetBothAggregateBountyCapUSDAndRetainable,
}
#[error_code]
pub enum AdoptionError {
    #[msg("No agreement found for adopter")]
    NoAgreement,

    #[msg("Adoption entry not found")]
    AdoptionEntryNotFound,

    #[msg("Adoption already exists")]
    AdoptionAlreadyExists,
}
#[error_code]
pub enum ValidationError {
    #[msg("Registry already initialized")]
    AlreadyInitialized,

    #[msg("Invalid protocol name")]
    InvalidProtocolName,

    #[msg("Invalid contact details")]
    InvalidContactDetails,

    #[msg("Invalid agreement URI")]
    InvalidAgreementUri,

    #[msg("Invalid new owner")]
    InvalidNewOwner,

    #[msg("Invalid owner key")]
    InvalidOwnerKey,

    #[msg("Invalid chain parameters")]
    InvalidChainParams,

    #[msg("Invalid account parameters")]
    InvalidAccountParams,

    #[msg("Max length exceeded")]
    MaxLengthExceeded,
}
#[error_code]
pub enum ProgramError {
    #[msg("Account already exists")]
    AccountAlreadyExists,
}
