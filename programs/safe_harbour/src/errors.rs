use anchor_lang::prelude::*;

/// Registry-related errors
#[error_code]
pub enum RegistryError {
    /// The registry has not been initialized
    #[msg("Registry has not been initialized")]
    RegistryNotInitialized,
    
    /// The caller is not the registry owner
    #[msg("Caller is not the registry owner")]
    NotRegistryOwner,
    
    /// The registry is already initialized
    #[msg("Registry is already initialized")]
    RegistryAlreadyInitialized,
    
    /// Invalid registry version
    #[msg("Invalid registry version")]
    InvalidRegistryVersion,

    
}

/// Agreement-related errors
#[error_code]
pub enum AgreementError {
    /// The agreement has not been initialized
    #[msg("Agreement has not been initialized")]
    AgreementNotInitialized,
    
    /// The caller is not the agreement owner
    #[msg("Caller is not the agreement owner")]
    NotAgreementOwner,
    
    /// The agreement is already initialized
    #[msg("Agreement is already initialized")]
    AgreementAlreadyInitialized,
    
    /// Invalid agreement parameters
    #[msg("Invalid agreement parameters")]
    InvalidAgreementParams,
    
    /// Agreement not found
    #[msg("Agreement not found")]
    AgreementNotFound,
}

/// Chain validation errors
#[error_code]
pub enum ChainError {
    /// Chain not found in the agreement
    #[msg("Chain not found in the agreement")]
    ChainNotFound,
    
    /// Chain not found by CAIP-2 ID
    #[msg("Chain not found by CAIP-2 ID")]
    ChainNotFoundByCaip2Id,
    
    /// Invalid CAIP-2 chain ID
    #[msg("Invalid CAIP-2 chain ID")]
    InvalidChainId,
    
    /// Duplicate CAIP-2 chain ID
    #[msg("Duplicate CAIP-2 chain ID")]
    DuplicateChainId,
    
    /// Chain not valid in registry
    #[msg("Chain not valid in registry")]
    ChainNotValidInRegistry,
}

/// Account-related errors
#[error_code]
pub enum AccountError {
    /// Account not found in the chain
    #[msg("Account not found in the chain")]
    AccountNotFound,
    
    /// Account not found by address
    #[msg("Account not found by address")]
    AccountNotFoundByAddress,
    
    /// Invalid account address
    #[msg("Invalid account address")]
    InvalidAccountAddress,
    
    /// Account already exists
    #[msg("Account already exists")]
    AccountAlreadyExists,
}

/// Bounty terms validation errors
#[error_code]
pub enum BountyError {
    /// Invalid bounty percentage (must be 0-100)
    #[msg("Invalid bounty percentage. Must be between 0 and 100")]
    InvalidBountyPercentage,
    
    /// Invalid bounty cap in USD
    #[msg("Invalid bounty cap in USD")]
    InvalidBountyCapUsd,
    
    /// Invalid aggregate bounty cap in USD
    #[msg("Invalid aggregate bounty cap in USD")]
    InvalidAggregateBountyCapUsd,
    
    /// Cannot set both aggregate bounty cap and retainable
    #[msg("Cannot set both aggregate bounty cap USD and retainable")]
    CannotSetBothAggregateBountyCapUSDAndRetainable,
    
    /// Invalid identity requirements
    #[msg("Invalid identity requirements")]
    InvalidIdentityRequirements,
    
    /// Invalid diligence requirements
    #[msg("Invalid diligence requirements")]
    InvalidDiligenceRequirements,
}

/// Adoption-related errors
#[error_code]
pub enum AdoptionError {
    /// No agreement found for adopter
    #[msg("No agreement found for adopter")]
    NoAgreement,
    
    /// Adoption entry not found
    #[msg("Adoption entry not found")]
    AdoptionEntryNotFound,
    
    /// Adoption head not found
    #[msg("Adoption head not found")]
    AdoptionHeadNotFound,
    
    /// Invalid adoption parameters
    #[msg("Invalid adoption parameters")]
    InvalidAdoptionParams,
    
    /// Adoption already exists
    #[msg("Adoption already exists")]
    AdoptionAlreadyExists,
}

/// Validation errors
#[error_code]
pub enum ValidationError {
    /// Invalid protocol name
    #[msg("Invalid protocol name")]
    InvalidProtocolName,
    
    /// Invalid contact details
    #[msg("Invalid contact details")]
    InvalidContactDetails,
    
    /// Invalid agreement URI
    #[msg("Invalid agreement URI")]
    InvalidAgreementUri,
    
    /// Invalid new owner
    #[msg("Invalid new owner")]
    InvalidNewOwner,
    
    /// Empty agreement parameters
    #[msg("Agreement parameters cannot be empty")]
    EmptyAgreementParams,
    
    /// Invalid chain parameters
    #[msg("Invalid chain parameters")]
    InvalidChainParams,
    
    /// Invalid account parameters
    #[msg("Invalid account parameters")]
    InvalidAccountParams,
}

/// Ownership transfer errors
#[error_code]
pub enum OwnershipError {
    /// Cannot transfer ownership to self
    #[msg("Cannot transfer ownership to self")]
    CannotTransferToSelf,
    
    /// Invalid new owner
    #[msg("Invalid new owner")]
    InvalidNewOwner,
    
    /// Ownership transfer not authorized
    #[msg("Ownership transfer not authorized")]
    OwnershipTransferNotAuthorized,
}

/// Space calculation errors
#[error_code]
pub enum SpaceError {
    /// Insufficient space for account
    #[msg("Insufficient space for account")]
    InsufficientSpace,
    
    /// Account space calculation failed
    #[msg("Account space calculation failed")]
    SpaceCalculationFailed,
    
    /// Account too large
    #[msg("Account too large")]
    AccountTooLarge,
}

/// Serialization errors
#[error_code]
pub enum SerializationError {
    /// Failed to serialize data
    #[msg("Failed to serialize data")]
    SerializationFailed,
    
    /// Failed to deserialize data
    #[msg("Failed to deserialize data")]
    DeserializationFailed,
    
    /// Invalid data format
    #[msg("Invalid data format")]
    InvalidDataFormat,
}

/// General program errors
#[error_code]
pub enum ProgramError {
    /// Unexpected error occurred
    #[msg("Unexpected error occurred")]
    UnexpectedError,
    
    /// Invalid instruction
    #[msg("Invalid instruction")]
    InvalidInstruction,
    
    /// Invalid account
    #[msg("Invalid account")]
    InvalidAccount,
    
    /// Invalid signer
    #[msg("Invalid signer")]
    InvalidSigner,
    
    /// Invalid program ID
    #[msg("Invalid program ID")]
    InvalidProgramId,
    
    /// Account already exists
    #[msg("Account already exists")]
    AccountAlreadyExists,
    
    /// Account does not exist
    #[msg("Account does not exist")]
    AccountDoesNotExist,
    
    /// Insufficient funds
    #[msg("Insufficient funds")]
    InsufficientFunds,
    
    /// Arithmetic overflow
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
    
    /// Arithmetic underflow
    #[msg("Arithmetic underflow")]
    ArithmeticUnderflow,
}
