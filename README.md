# Safe Harbor Registry V2 - Solana Implementation

A comprehensive Solana program implementing the Safe Harbor Registry V2 specification, providing a decentralized registry for bug bounty agreements and CAIP-2 chain validation.

## Overview

The Safe Harbor Registry V2 is a decentralized system that allows protocols to adopt standardized bug bounty agreements, providing transparency and legal clarity for security researchers (whitehats) and protocol teams. This Solana implementation provides the same functionality as the Ethereum version but optimized for the Solana ecosystem.

## Features

### 🏛️ Registry Management
- **Centralized Registry**: Single source of truth for all Safe Harbor adoptions
- **Owner Controls**: Registry owner can manage valid CAIP-2 chains
- **Version Management**: Clear versioning system (v1.1.0)

### 📋 Agreement Management
- **Ownership Controls**: Agreements can be owned and transferred
- **Mutable Terms**: Agreement owners can update terms as needed
- **Comprehensive Validation**: All agreement parameters are validated

### 🌐 Multi-Chain Support
- **CAIP-2 Chain Validation**: Support for standardized chain identification
- **Chain Management**: Add/remove valid chains dynamically
- **Cross-Chain Compatibility**: Works with any CAIP-2 compliant chain

### 💰 Bounty Terms
- **Flexible Bounty Structure**: Configurable percentage and cap limits
- **Identity Requirements**: Anonymous, Pseudonymous, or Named whitehats
- **Aggregate Caps**: Optional total bounty limits across all whitehats

### 📞 Contact Management
- **Pre-Notification Contacts**: Required contact information for whitehats
- **Multiple Contacts**: Support for multiple contact methods
- **Validation**: Contact information is validated for completeness

## Architecture

### Core Components

1. **Registry**: Main registry account managing all adoptions and valid chains
2. **Agreement**: Individual agreement accounts with ownership controls
3. **Adoption**: Links between adopters and their agreements
4. **AdoptionHead**: O(1) lookup for current agreement per adopter

### Account Structure

```
Registry (PDA: "registry_v2")
├── Owner: Pubkey
├── Agreements: AccountMap
└── ValidChains: Vec<String>

Agreement (Regular Account)
├── Owner: Pubkey
├── ProtocolName: String
├── ContactDetails: Vec<Contact>
├── Chains: Vec<Chain>
├── BountyTerms: BountyTerms
└── AgreementURI: String

AdoptionEntry (PDA: "adoption_v2", adopter, agreement)
└── Agreement: Pubkey

AdoptionHead (PDA: "adoption_head", adopter)
└── Agreement: Pubkey
```

## Installation

### Prerequisites

- Rust 1.70+
- Solana CLI 1.17+
- Anchor Framework 0.29+

### Setup

1. Clone the repository:
```bash
git clone <repository-url>
cd safe-harbour-project/safe-harbour
```

2. Install dependencies:
```bash
anchor build
```

3. Run tests:
```bash
anchor test
```

## Usage

### Initializing the Registry

```rust
// Initialize the registry with an owner
initialize_registry(ctx, owner_pubkey)?;
```

### Setting Valid Chains

```rust
// Add valid CAIP-2 chains
set_valid_chains(ctx, vec![
    "eip155:1".to_string(),      // Ethereum Mainnet
    "eip155:137".to_string(),     // Polygon
    "solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp".to_string(), // Solana Mainnet
])?;
```

### Creating an Agreement

```rust
let agreement_params = AgreementInitParams {
    protocol_name: "My Protocol".to_string(),
    contact_details: vec![Contact {
        name: "Security Team".to_string(),
        contact: "security@myprotocol.com".to_string(),
    }],
    chains: vec![Chain {
        asset_recovery_address: "0x1234567890123456789012345678901234567890".to_string(),
        accounts: vec![AccountInScope {
            account_address: "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd".to_string(),
            child_contract_scope: ChildContractScope::All,
        }],
        caip2_chain_id: "eip155:1".to_string(),
    }],
    bounty_terms: BountyTerms {
        bounty_percentage: 10,
        bounty_cap_usd: 100000,
        retainable: false,
        identity: IdentityRequirements::Pseudonymous,
        diligence_requirements: "Must provide proof of identity".to_string(),
        aggregate_bounty_cap_usd: 500000,
    },
    agreement_uri: "ipfs://QmYourAgreementHash".to_string(),
};

create_agreement(ctx, agreement_params, owner_pubkey)?;
```

### Adopting an Agreement

```rust
// Adopt an existing agreement
adopt_safe_harbor(ctx)?;
```

### Creating and Adopting in One Transaction

```rust
// Create and adopt in a single transaction
create_and_adopt_agreement(ctx, agreement_params, owner_pubkey)?;
```

## API Reference

### Registry Functions

- `initialize_registry(ctx, owner)` - Initialize the registry
- `set_valid_chains(ctx, chains)` - Add valid CAIP-2 chains
- `set_invalid_chains(ctx, chains)` - Remove valid CAIP-2 chains
- `is_chain_valid(ctx, chain_id)` - Check if a chain is valid
- `get_valid_chains(ctx)` - Get all valid chains

### Agreement Functions

- `create_agreement(ctx, params, owner)` - Create a new agreement
- `set_protocol_name(ctx, name)` - Update protocol name
- `set_contact_details(ctx, contacts)` - Update contact details
- `add_chains(ctx, chains)` - Add chains to agreement
- `set_chains(ctx, chains)` - Set all chains for agreement
- `remove_chains(ctx, chain_ids)` - Remove chains from agreement
- `add_accounts(ctx, chain_id, accounts)` - Add accounts to a chain
- `remove_accounts(ctx, chain_id, addresses)` - Remove accounts from a chain
- `set_bounty_terms(ctx, terms)` - Update bounty terms
- `set_agreement_uri(ctx, uri)` - Update agreement URI
- `transfer_ownership(ctx, new_owner)` - Transfer agreement ownership

### Adoption Functions

- `adopt_safe_harbor(ctx)` - Adopt an agreement
- `create_and_adopt_agreement(ctx, params, owner)` - Create and adopt in one transaction

### Query Functions

- `get_agreement_details(ctx)` - Get agreement details
- `get_agreement_for_adopter(ctx)` - Get current agreement for adopter
- `get_agreement_by_pda(ctx)` - Get agreement by adoption PDA
- `version()` - Get program version

## Events

The program emits comprehensive events for all state changes:

- `RegistryInitialized` - Registry creation
- `ChainValidityChanged` - Chain validation changes
- `SafeHarborAdopted` - Agreement adoptions
- `AgreementCreated` - Agreement creation
- `AgreementUpdated` - Agreement modifications
- `AgreementOwnershipTransferred` - Ownership transfers
- `ProtocolNameUpdated` - Protocol name changes
- `ContactDetailsUpdated` - Contact detail changes
- `ChainsUpdated` - Chain modifications
- `AccountsUpdated` - Account changes
- `BountyTermsUpdated` - Bounty term changes
- `AgreementUriUpdated` - URI changes

## Error Handling

The program includes comprehensive error handling with specific error types:

- `RegistryError` - Registry-related errors
- `AgreementError` - Agreement-related errors
- `ChainError` - Chain validation errors
- `AccountError` - Account-related errors
- `BountyError` - Bounty term validation errors
- `AdoptionError` - Adoption-related errors
- `ValidationError` - General validation errors
- `OwnershipError` - Ownership transfer errors
- `SpaceError` - Account space errors
- `SerializationError` - Data serialization errors
- `ProgramError` - General program errors

## Security Considerations

### Access Control
- Registry owner has admin privileges
- Agreement owners can only modify their own agreements
- All operations require proper authorization

### Validation
- All input parameters are validated
- CAIP-2 chain IDs are validated for format
- Account addresses are validated per chain type
- Bounty terms are validated for consistency

### Space Management
- Dynamic space calculation for accounts
- Buffer allocation for future growth
- Protection against account size limits

## Testing

The program includes comprehensive tests covering:

- Unit tests for all modules
- Integration tests for complete workflows
- Error condition testing
- Edge case validation
- Space calculation verification

Run tests with:
```bash
anchor test
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Support

For questions and support:
- Visit [Security Alliance](https://www.securityalliance.org)
- Join our Discord community
- Open an issue on GitHub

## Version History

- **v1.1.0** - Initial Solana implementation with full V2 compatibility
- **v1.0.0** - Ethereum implementation baseline

## Acknowledgments

- Security Alliance for the Safe Harbor initiative
- Solana Foundation for the Anchor framework
- The broader Solana developer community