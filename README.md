# Safe Harbor Registry V2 - Solana Implementation

Protocols publish and adopt standardized bug bounties.

## Features

### Agreement Management
- **Create agreements**: Establish standardized bug bounty terms with configurable bounty percentages, caps, and requirements
- **Update agreements**: Modify protocol names, contact details, bounty terms, or agreement URIs
- **Flexible addressing**: Use nonce-based PDAs to allow multiple agreements per owner

### Adoption System
- **Adopt agreements**: Protocols can formally adopt existing agreements to participate in the bounty program.

## Build & Deploy
```bash
$ anchor build
$ anchor deploy
```

## Testing
```bash
$ anchor test
```
