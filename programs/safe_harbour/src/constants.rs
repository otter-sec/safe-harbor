/// Seed prefix for registry account
pub const REGISTRY_SEED: &[u8] = b"registry_v2";

/// Seed prefix for adoption accounts
pub const ADOPTION_SEED: &[u8] = b"adoption_v2";

/// Base space for account discriminators
pub const DISCRIMINATOR_SIZE: usize = 8;

/// Base space for public key storage
pub const PUBKEY_SIZE: usize = 32;

/// Base space for registry account
pub const REGISTRY_BASE_SPACE: usize = DISCRIMINATOR_SIZE + PUBKEY_SIZE;

/// Initial space allocation for registry account
pub const REGISTRY_INITIAL_SPACE: usize = REGISTRY_BASE_SPACE + 1024; // Base + 1KB buffer

/// Base space for agreement account
pub const AGREEMENT_BASE_SPACE: usize = DISCRIMINATOR_SIZE + PUBKEY_SIZE;

/// Initial space allocation for agreement account
pub const AGREEMENT_INITIAL_SPACE: usize = AGREEMENT_BASE_SPACE + 1024; // Base + 1KB buffer

/// Space for adoption account
pub const ADOPTION_SPACE: usize = DISCRIMINATOR_SIZE + PUBKEY_SIZE;

/// Maximum number of chains allowed per agreement
pub const MAX_CHAINS_PER_AGREEMENT: usize = 10;

/// Maximum length for protocol name
pub const MAX_PROTOCOL_NAME_LENGTH: usize = 100;

/// Maximum length for agreement URI
pub const MAX_AGREEMENT_URI_LENGTH: usize = 200;

/// Maximum length for contact details
pub const MAX_CONTACT_DETAILS_LENGTH: usize = 500;
