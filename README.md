# Safe Harbor Registry V2 - Solana Implementation

Protocols publish and adopt standardized bug bounties.

## What you can do: 
- create/update agreements, manage in-scope accounts, update bounty terms/URI/contacts/name, transfer ownership.
- adopt the latest agreement, query current agreement for an adopter.
- What protocol does: input validation, access control, and event emission for all state changes.

## Build & Deploy
```bash
$ anchor build
$ anchor deploy
```

## Test Localnet
```bash
$ solana-test-validator --reset
$ solana airdrop 100    
$ solana balance
$ anchor test --skip-local-validator
```