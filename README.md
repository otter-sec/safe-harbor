# Safe Harbor Registry V2 - Solana Implementation

Protocols publish and adopt standardized bug bounties.

- What you can do: create/update agreements, manage in-scope accounts, update bounty terms/URI/contacts/name, transfer ownership, adopt the latest agreement, query current agreement for an adopter.
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

Minimal flow (Rust)
```rust

let data = AgreementData {
    ..Default::default() 
};
create_or_update_agreement(ctx, data, owner_pubkey, AgreementUpdateType::InitializeOrUpdate, init_nonce)?;

// subsequent targeted updates
create_or_update_agreement(ctx, AgreementData { protocol_name: "My Protocol".into(), ..Default::default() }, owner_pubkey, AgreementUpdateType::ProtocolName, init_nonce)?;

// adopt
create_or_update_adoption(ctx)?;
```
