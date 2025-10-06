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
    pub adopter: Pubkey,              // Added for efficient querying
    pub agreement: Pubkey,
    pub caip2_chain_id: String,
    pub asset_recovery_address: String,
    pub accounts: Vec<AccountInScope>, // Max 20 contracts per adoption
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
# Run all tests (11 test cases: 7 core + 4 query tests)
anchor test
```

## Querying Adoptions

### Fetch All Adoptions by Adopter

The `adopter` field is now stored directly in the account data at offset 8, enabling efficient on-chain filtering.

```typescript
import { PublicKey } from "@solana/web3.js";
import { createHash } from "crypto";
import * as bs58 from "bs58";

// Get all adoptions for a specific adopter using getProgramAccounts
async function fetchAdoptionsByAdopter(
  connection: Connection,
  programId: PublicKey,
  adopter: PublicKey
) {
  // Get the discriminator for the Adopt account type
  const adoptDiscriminator = Buffer.from(
    createHash("sha256").update("account:Adopt").digest()
  ).subarray(0, 8);

  const accounts = await connection.getProgramAccounts(programId, {
    filters: [
      {
        // Filter by account discriminator (first 8 bytes)
        memcmp: {
          offset: 0,
          bytes: bs58.encode(adoptDiscriminator),
        },
      },
      {
        // Filter by adopter field at offset 8 (after discriminator)
        memcmp: {
          offset: 8,
          bytes: adopter.toBase58(),
        },
      },
    ],
  });

  // Decode and return the adoption accounts
  return accounts.map((account) => ({
    publicKey: account.pubkey,
    account: program.coder.accounts.decode("adopt", account.account.data),
  }));
}

// Get a specific adoption for a known chain
async function getAdoptionForChain(
  program: Program<SafeHarbor>,
  adopter: PublicKey,
  chainId: string
): Promise<Adopt | null> {
  const chainIdHash = createHash("sha256").update(chainId).digest();
  const [adoptionPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("adopt_v2"), adopter.toBuffer(), chainIdHash],
    program.programId
  );

  try {
    const adoption = await program.account.adopt.fetch(adoptionPda);
    return adoption;
  } catch {
    return null; // Account doesn't exist
  }
}
```

### Filter by Contract/Program Address (Client-Side)

Since the `accounts` field is a variable-length Vec, filtering by contract address must be done client-side after fetching:

```typescript
// Filter adoptions that include a specific contract address
function filterAdoptionsByProgramAddress(
  adoptions: any[],
  programAddress: string
) {
  return adoptions.filter((adoption) =>
    adoption.account.accounts.some(
      (account: any) => account.accountAddress === programAddress
    )
  );
}

// Complete workflow: Fetch by adopter, then filter by program
async function getAdoptionsWithProgram(
  connection: Connection,
  programId: PublicKey,
  adopter: PublicKey,
  contractAddress: string
) {
  // Step 1: Fetch all adoptions by adopter (on-chain filter)
  const allAdoptions = await fetchAdoptionsByAdopter(
    connection,
    programId,
    adopter
  );

  // Step 2: Filter by contract address (client-side)
  return filterAdoptionsByProgramAddress(allAdoptions, contractAddress);
}
```
