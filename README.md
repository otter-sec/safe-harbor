# Safe Harbor V2 - Solana Implementation

A Solana-based implementation of the [SEAL Safe Harbor Agreement](https://github.com/security-alliance/safe-harbor), providing legal protection for whitehat security researchers during active cryptocurrency exploits.

## Overview

Safe Harbor enables protocols to establish clear bug bounty terms and security researcher protections on-chain. The implementation features a two-account model optimized for Solana's rent economics:

- **Agreement Accounts**: Store bounty terms, contact details, and security policy
- **Adoption Accounts**: Chain-specific adoptions with asset recovery addresses and in-scope contracts

## Key Features

### 🔐 Agreement Management
- **Create**: Establish standardized bug bounty terms with configurable percentages, caps, and identity requirements
- **Modular Updates**: Update specific fields (protocol name, contacts, bounty terms, or URI) without full rewrites
- **Multiple Agreements**: Use nonce-based PDAs to create multiple agreements per owner

### 🌐 Multi-Chain Adoption
- **CAIP-2 Support**: Adopt agreements on multiple chains using standard chain identifiers
- **Chain-Specific Settings**: Set unique asset recovery addresses and in-scope accounts per chain
- **Optimized PDAs**: SHA256 hashing of chain IDs to fit Solana's 32-byte PDA seed limit

## Architecture

### Agreement Account
```rust
pub struct AgreementData {
    pub owner: Pubkey,
    pub protocol_name: String,
    pub contact_details: Vec<Contact>,
    pub bounty_terms: BountyTerms,
    pub agreement_uri: String,
}
```

### Adoption Account
```rust
pub struct Adopt {
    pub agreement: Pubkey,
    pub caip2_chain_id: String,
    pub asset_recovery_address: String,
    pub accounts: Vec<AccountInScope>,
}
```

### PDA Derivation
- **Agreement**: `[b"agreement_v2", signer, nonce]`
- **Adoption**: `[b"adopt_v2", adopter, SHA256(chain_id)]`

## Build & Deploy

```bash
# Build the program
anchor build

# Deploy to devnet/mainnet
anchor deploy
```

## Testing

```bash
# Run all tests (7 test cases)
anchor test
```