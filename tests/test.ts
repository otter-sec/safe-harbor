import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { Buffer } from "buffer";
import { BN } from "bn.js";
import { assert } from "chai";
import { SafeHarbor } from "../target/types/safe_harbor";
import { createHash } from "crypto";

// Seeds
const AGREEMENT_SEED = Buffer.from("agreement_v2");
const ADOPTION_SEED = Buffer.from("adopt_v2");

describe("safe_harbor v2", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SafeHarbor as Program<SafeHarbor>;
  const provider = anchor.getProvider();
  const owner = (provider.wallet as anchor.Wallet).payer;

  let agreement_pda: PublicKey;
  let adoption_pda: PublicKey;

  before(async () => {
    const initNonce = new BN(1);
    // CAIP-2 chain ID for Solana mainnet (Reference: https://namespaces.chainagnostic.org/solana/caip2)
    const chainId = "solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp";

    // Hash the chain ID to fit within Solana's 32-byte PDA seed limit
    const chainIdHash = createHash("sha256").update(chainId).digest();

    // Derive PDAs - Program Derived Addresses for deterministic account creation
    [agreement_pda] = PublicKey.findProgramAddressSync(
      [
        AGREEMENT_SEED,
        owner.publicKey.toBuffer(),
        Buffer.from(initNonce.toArrayLike(Buffer, "le", 8)),
      ],
      program.programId
    );

    [adoption_pda] = PublicKey.findProgramAddressSync(
      [ADOPTION_SEED, owner.publicKey.toBuffer(), chainIdHash],
      program.programId
    );

    console.log(`Agreement PDA: ${agreement_pda.toBase58()}`);
    console.log(`Adoption PDA: ${adoption_pda.toBase58()}`);
  });

  it("Create Agreement", async () => {
    const initNonce = new BN(1);
    const agreementData = {
      owner: owner.publicKey,
      protocolName: "Safe harbor Protocol",
      contactDetails: [
        { name: "Security Team", contact: "security@example.com" },
      ],
      bountyTerms: {
        bountyPercentage: new BN(10),
        bountyCapUsd: new BN(100000),
        retainable: false,
        identity: { anonymous: {} },
        diligenceRequirements: "Responsible disclosure required",
        aggregateBountyCapUsd: new BN(0),
      },
      agreementUri: "ipfs://QmAgreementHash1",
    };

    const tx = await program.methods
      .createOrUpdateAgreement(initNonce, agreementData, owner.publicKey, 0)
      .accounts({
        agreement: agreement_pda,
        signer: owner.publicKey,
        systemProgram: SystemProgram.programId,
      } as any)
      .rpc();

    // wait for confirmation
    const latestBlockHash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: tx,
    });

    // Fetch and verify the created agreement
    const agreementAccount = await program.account.agreementData.fetch(
      agreement_pda
    );

    // Assert agreement data
    assert.equal(agreementAccount.protocolName, "Safe harbor Protocol");
    assert.equal(
      agreementAccount.bountyTerms.bountyPercentage.toString(),
      "10"
    );
    assert.equal(
      agreementAccount.bountyTerms.bountyCapUsd.toString(),
      "100000"
    );
    assert.equal(agreementAccount.owner.toString(), owner.publicKey.toString());
  });

  it("Adopt Agreement", async () => {
    const chainId = "solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp";
    const chainIdHash = createHash("sha256").update(chainId).digest();
    const assetRecoveryAddress = owner.publicKey.toBase58();
    const accounts = [
      {
        accountAddress: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
        childContractScope: { none: {} },
      },
      {
        accountAddress: "11111111111111111111111111111111",
        childContractScope: { all: {} },
      },
    ];

    const tx = await program.methods
      .createOrUpdateAdoption(
        chainId,
        Array.from(chainIdHash),
        0, // InitializeOrUpdate
        null, // new_agreement (not needed for initial creation)
        assetRecoveryAddress,
        accounts,
        null // no addresses to remove
      )
      .accounts({
        adoption: adoption_pda,
        adopter: owner.publicKey,
        agreement: agreement_pda,
        systemProgram: SystemProgram.programId,
      } as any)
      .rpc();

    // wait for confirmation
    const latestBlockHash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: tx,
    });

    // Fetch and verify the created adoption
    const adoptionAccount = await program.account.adopt.fetch(adoption_pda);

    // Assert adoption data
    assert.equal(
      adoptionAccount.agreement.toString(),
      agreement_pda.toString()
    );
    assert.equal(adoptionAccount.caip2ChainId, chainId);
    assert.equal(adoptionAccount.assetRecoveryAddress, assetRecoveryAddress);
    assert.equal(adoptionAccount.accounts.length, 2);
    assert.equal(
      adoptionAccount.accounts[0].accountAddress,
      "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
    );
  });

  it("Reject adoption with invalid chain_id hash", async () => {
    const chainId = "solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp";
    const wrongHash = createHash("sha256").update("wrong:chain").digest();
    const assetRecoveryAddress = owner.publicKey.toBase58();
    const accounts = [];

    // Derive PDA with the wrong hash to match what we're passing
    const [wrong_adoption_pda] = PublicKey.findProgramAddressSync(
      [ADOPTION_SEED, owner.publicKey.toBuffer(), wrongHash],
      program.programId
    );

    try {
      await program.methods
        .createOrUpdateAdoption(
          chainId, // Actual chain ID
          Array.from(wrongHash), // Wrong hash - doesn't match chainId
          0, // InitializeOrUpdate
          null, // new_agreement
          assetRecoveryAddress,
          accounts,
          null // no addresses to remove
        )
        .accounts({
          adoption: wrong_adoption_pda, // Use PDA derived with wrong hash
          adopter: owner.publicKey,
          agreement: agreement_pda,
          systemProgram: SystemProgram.programId,
        } as any)
        .rpc();

      assert.fail("Should have failed with invalid hash");
    } catch (error: any) {
      // Expect InvalidAccountParams error (hash validation failed)
      assert.include(
        error.toString(),
        "InvalidAccountParams",
        "Should reject mismatched hash"
      );
    }
  });

  it("Add accounts to adoption", async () => {
    const chainId = "solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp";
    const chainIdHash = createHash("sha256").update(chainId).digest();

    const newAccounts = [
      {
        accountAddress: "9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin", // Serum program
        childContractScope: { existingOnly: {} },
      },
    ];

    const tx = await program.methods
      .createOrUpdateAdoption(
        chainId,
        Array.from(chainIdHash),
        2, // AddAccounts
        null, // no agreement change
        null, // no recovery address change
        newAccounts,
        null // no accounts to remove
      )
      .accounts({
        adoption: adoption_pda,
        adopter: owner.publicKey,
        agreement: agreement_pda,
        systemProgram: SystemProgram.programId,
      } as any)
      .rpc();

    await provider.connection.confirmTransaction(tx);

    const adoptionAccount = await program.account.adopt.fetch(adoption_pda);
    assert.equal(adoptionAccount.accounts.length, 3); // Original 2 + 1 new
    assert.equal(
      adoptionAccount.accounts[2].accountAddress,
      "9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin"
    );
  });

  it("Remove accounts from adoption", async () => {
    const chainId = "solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp";
    const chainIdHash = createHash("sha256").update(chainId).digest();

    const addressesToRemove = ["11111111111111111111111111111111"];

    const tx = await program.methods
      .createOrUpdateAdoption(
        chainId,
        Array.from(chainIdHash),
        3, // RemoveAccounts
        null,
        null,
        null,
        addressesToRemove
      )
      .accounts({
        adoption: adoption_pda,
        adopter: owner.publicKey,
        agreement: agreement_pda,
        systemProgram: SystemProgram.programId,
      } as any)
      .rpc();

    await provider.connection.confirmTransaction(tx);

    const adoptionAccount = await program.account.adopt.fetch(adoption_pda);
    assert.equal(adoptionAccount.accounts.length, 2); // 3 - 1 removed
    assert.isFalse(
      adoptionAccount.accounts.some(
        (a) => a.accountAddress === "11111111111111111111111111111111"
      )
    );
  });

  it("Update asset recovery address (only - all other fields unchanged)", async () => {
    const chainId = "solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp";
    const chainIdHash = createHash("sha256").update(chainId).digest();
    const newRecoveryAddress = "NewRecoveryAddress123456789012345678";

    // Fetch state before update
    const beforeUpdate = await program.account.adopt.fetch(adoption_pda);
    const originalAgreement = beforeUpdate.agreement;
    const originalAccountsCount = beforeUpdate.accounts.length;

    // Update ONLY the recovery address by passing null for all other params
    const tx = await program.methods
      .createOrUpdateAdoption(
        chainId,
        Array.from(chainIdHash),
        4, // UpdateAssetRecoveryAddress
        null, // new_agreement = None (no change)
        newRecoveryAddress, // new_asset_recovery_address = Some(value)
        null, // accounts_to_add = None (no change)
        null  // account_addresses_to_remove = None (no change)
      )
      .accounts({
        adoption: adoption_pda,
        adopter: owner.publicKey,
        agreement: agreement_pda,
        systemProgram: SystemProgram.programId,
      } as any)
      .rpc();

    await provider.connection.confirmTransaction(tx);

    const afterUpdate = await program.account.adopt.fetch(adoption_pda);

    // Verify ONLY recovery address changed
    assert.equal(
      afterUpdate.assetRecoveryAddress,
      newRecoveryAddress,
      "Recovery address should be updated"
    );

    // Verify all other fields remain unchanged
    assert.equal(
      afterUpdate.agreement.toString(),
      originalAgreement.toString(),
      "Agreement should remain unchanged"
    );
    assert.equal(
      afterUpdate.accounts.length,
      originalAccountsCount,
      "Accounts list should remain unchanged"
    );
    assert.equal(
      afterUpdate.caip2ChainId,
      chainId,
      "Chain ID should remain unchanged"
    );
  });

  it("Update Agreement", async () => {
    const initNonce = new BN(1);
    const updatedAgreementData = {
      owner: owner.publicKey,
      protocolName: "Safe harbor Protocol v2",
      contactDetails: [
        // Updated contact details
        { name: "Security Team", contact: "security@example.com" },
        { name: "Bug Bounty Lead", contact: "bounty@example.com" },
      ],
      bountyTerms: {
        bountyPercentage: new BN(15), // Updated bounty percentage
        bountyCapUsd: new BN(250000), // Updated bounty cap
        retainable: true, // Updated retainable flag
        identity: { anonymous: {} },
        diligenceRequirements: "Responsible disclosure required",
        aggregateBountyCapUsd: new BN(0),
      },
      agreementUri: "ipfs://QmAgreementHash1",
    };

    const tx = await program.methods
      .createOrUpdateAgreement(
        initNonce,
        updatedAgreementData,
        owner.publicKey,
        0 // update_type: 0 = InitializeOrUpdate
      )
      .accounts({
        agreement: agreement_pda,
        signer: owner.publicKey,
        systemProgram: SystemProgram.programId,
      } as any)
      .rpc();

    // wait for confirmation
    const latestBlockHash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: tx,
    });

    // Fetch and verify the updated agreement
    const updatedAgreementAccount = await program.account.agreementData.fetch(
      agreement_pda
    );

    // Assert updated agreement data
    assert.equal(
      updatedAgreementAccount.protocolName,
      "Safe harbor Protocol v2"
    );
    assert.equal(
      updatedAgreementAccount.bountyTerms.bountyPercentage.toString(),
      "15"
    );
    assert.equal(
      updatedAgreementAccount.bountyTerms.bountyCapUsd.toString(),
      "250000"
    );
    assert.equal(updatedAgreementAccount.bountyTerms.retainable, true);
    assert.equal(updatedAgreementAccount.contactDetails.length, 2);
  });
});
