use crate::utils::types::AccountInScope;
use anchor_lang::prelude::*;

// Asset recovery address max length
// Ethereum/EVM: 42 chars, Solana: 44 chars, Bitcoin SegWit: 62 chars
// Cardano (Shelley): 103-105 chars, Polkadot: 48 chars
// Set to 128 to safely cover all major chains including Cardano
pub const MAX_ASSET_RECOVERY_ADDR_LEN: usize = 128;

pub const MAX_ACCOUNTS_PER_CHAIN: usize = 50;

// CAIP-2 Chain ID max length
// Shortest: ~9 chars (eip155:1)
// Longest: ~43 chars (Bitcoin/Polkadot style)
// Set to 64 for future-proofing and extra namespaces
pub const MAX_CHAIN_ID_LEN: usize = 64;

/// Adoption record linking an adopter to a Safe Harbor agreement for a specific chain.
/// Each adopter can have multiple Adopt accounts (one per chain).
#[account]
pub struct Adopt {
    /// The public key of the Agreement being adopted
    pub agreement: Pubkey,

    /// CAIP-2 chain identifier (e.g., "eip155:1" for Ethereum mainnet)
    /// This is also used as part of the PDA seed to allow multi-chain adoptions
    pub caip2_chain_id: String,

    /// Address where recovered assets should be sent on this specific chain
    /// Format varies by chain (e.g., 0x... for Ethereum, base58 for Solana, bech32 for Cardano)
    pub asset_recovery_address: String,

    /// List of smart contract addresses covered by this Safe Harbor agreement on this chain
    /// Each account specifies which contracts are in-scope and how child contracts are handled
    pub accounts: Vec<AccountInScope>,
}

/// Types of updates that can be made to an Adopt account
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum AdoptUpdateType {
    /// Initialize a new adoption or perform a full update (used on account creation)
    InitializeOrUpdate,

    /// Replace all fields at once (full update)
    ReplaceAll,

    /// Add new accounts to the existing list
    AddAccounts,

    /// Remove specific accounts by address
    RemoveAccounts,

    /// Update only the asset recovery address
    UpdateAssetRecoveryAddress,

    /// Update the referenced agreement
    UpdateAgreement,
}

impl Adopt {
    /// PDA seed prefix for adoption accounts
    pub const ADOPT_SEED: &'static [u8] = b"adopt_v2";

    /// Maximum space allocated for an Adopt account
    /// This provides sufficient space for max-length fields:
    /// - 8 (discriminator)
    /// - 32 (agreement pubkey)
    /// - 4 + 64 (chain_id with max length)
    /// - 4 + 128 (asset_recovery_address with max length)
    /// - 4 + (50 * 69) (max accounts, each ~69 bytes)
    /// - 256 (buffer for account reallocation and future growth)
    /// - Total: ~3,900 bytes
    pub const INITIAL_SPACE: usize = 8
        + 32
        + 4
        + MAX_CHAIN_ID_LEN
        + 4
        + MAX_ASSET_RECOVERY_ADDR_LEN
        + 4
        + (MAX_ACCOUNTS_PER_CHAIN * 69)
        + 256;
}
