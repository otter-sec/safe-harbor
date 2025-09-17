
pub const REGISTRY_SEED: &[u8] = b"registry_v2";

pub const ADOPTION_SEED: &[u8] = b"adoption_v2";

pub const DISCRIMINATOR_SIZE: usize = 8;

pub const PUBKEY_SIZE: usize = 32;

pub const REGISTRY_BASE_SPACE: usize = DISCRIMINATOR_SIZE + PUBKEY_SIZE;

pub const REGISTRY_INITIAL_SPACE: usize = REGISTRY_BASE_SPACE + 1024; // Base + 1KB buffer

pub const AGREEMENT_BASE_SPACE: usize = DISCRIMINATOR_SIZE + PUBKEY_SIZE;

pub const AGREEMENT_INITIAL_SPACE: usize = AGREEMENT_BASE_SPACE + 1024; // Base + 1KB buffer

pub const ADOPTION_SPACE: usize = DISCRIMINATOR_SIZE + PUBKEY_SIZE;

pub const MAX_CHAINS_PER_AGREEMENT: usize = 10;

pub const MAX_PROTOCOL_NAME_LENGTH: usize = 100;

pub const MAX_AGREEMENT_URI_LENGTH: usize = 200;

pub const MAX_CONTACT_DETAILS_LENGTH: usize = 500;
