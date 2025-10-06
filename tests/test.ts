import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { Buffer } from "buffer";
import { BN } from "bn.js";
import { assert } from "chai";
import { SafeHarbor } from "../target/types/safe_harbor";
import { createHash } from "crypto";
import * as bs58 from "bs58";

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

  // ============================================================================
  // Helper Functions for Querying Adoptions
  // ============================================================================

  /**
   * Fetch all adoption accounts for a specific adopter
   * @param adopter - The public key of the adopter
   * @returns Array of adoption accounts with their public keys
   */
  async function fetchAdoptionsByAdopter(adopter: PublicKey) {
    // Get the discriminator for the Adopt account type
    const adoptDiscriminator = Buffer.from(
      createHash("sha256").update("account:Adopt").digest()
    ).subarray(0, 8);

    const accounts = await provider.connection.getProgramAccounts(
      program.programId,
      {
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
      }
    );

    // Decode and return the adoption accounts
    return accounts.map((account) => ({
      publicKey: account.pubkey,
      account: program.coder.accounts.decode("adopt", account.account.data),
    }));
  }

  /**
   * Filter adoption accounts by a specific contract/program address
   * @param adoptions - Array of adoption accounts
   * @param programAddress - The contract/program address to filter by
   * @returns Filtered adoptions that include the specified program address
   */
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

  // ============================================================================
  // Query Tests
  // ============================================================================

  describe("Query Adoptions", () => {
    it("Fetch all adoptions by adopter", async () => {
      // Fetch all adoptions for the owner
      const adoptions = await fetchAdoptionsByAdopter(owner.publicKey);

      // Should have at least 1 adoption (from previous tests)
      assert.isAtLeast(adoptions.length, 1, "Should have at least 1 adoption");

      console.log(`\nFound ${adoptions.length} adoption(s) for adopter:`);
      adoptions.forEach((adoption, index) => {
        console.log(`\nAdoption #${index + 1}:`);
        console.log(`  PDA: ${adoption.publicKey.toBase58()}`);
        console.log(`  Chain ID: ${adoption.account.caip2ChainId}`);
        console.log(`  Adopter: ${adoption.account.adopter.toBase58()}`);
        console.log(
          `  Agreement: ${adoption.account.agreement.toBase58()}`
        );
        console.log(
          `  Asset Recovery: ${adoption.account.assetRecoveryAddress}`
        );
        console.log(`  Contracts: ${adoption.account.accounts.length}`);
        adoption.account.accounts.forEach((account: any, i: number) => {
          console.log(`    ${i + 1}. ${account.accountAddress}`);
        });
      });

      // Verify the adopter field matches
      adoptions.forEach((adoption) => {
        assert.equal(
          adoption.account.adopter.toBase58(),
          owner.publicKey.toBase58(),
          "Adopter field should match queried adopter"
        );
      });
    });

    it("Filter adoptions by specific program address", async () => {
      // Fetch all adoptions for the owner
      const adoptions = await fetchAdoptionsByAdopter(owner.publicKey);

      // Filter by the Token program address
      const tokenProgramAddress = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
      const filtered = filterAdoptionsByProgramAddress(
        adoptions,
        tokenProgramAddress
      );

      // Should have at least 1 adoption with Token program
      assert.isAtLeast(
        filtered.length,
        1,
        "Should have at least 1 adoption with Token program"
      );

      console.log(
        `\nFound ${filtered.length} adoption(s) including ${tokenProgramAddress}:`
      );
      filtered.forEach((adoption, index) => {
        console.log(`\nAdoption #${index + 1}:`);
        console.log(`  PDA: ${adoption.publicKey.toBase58()}`);
        console.log(`  Chain ID: ${adoption.account.caip2ChainId}`);
        console.log(
          `  Contracts including ${tokenProgramAddress}:`
        );
        adoption.account.accounts.forEach((account: any, i: number) => {
          if (account.accountAddress === tokenProgramAddress) {
            console.log(`    ✓ ${account.accountAddress}`);
          }
        });
      });

      // Verify all filtered adoptions contain the program address
      filtered.forEach((adoption) => {
        const hasProgram = adoption.account.accounts.some(
          (account: any) => account.accountAddress === tokenProgramAddress
        );
        assert.isTrue(
          hasProgram,
          `Adoption should contain ${tokenProgramAddress}`
        );
      });
    });

    it("Filter adoptions by Serum program address", async () => {
      // Fetch all adoptions for the owner
      const adoptions = await fetchAdoptionsByAdopter(owner.publicKey);

      // Filter by Serum program (added in "Add accounts to adoption" test)
      const serumProgramAddress =
        "9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin";
      const filtered = filterAdoptionsByProgramAddress(
        adoptions,
        serumProgramAddress
      );

      // Should have at least 1 adoption with Serum program
      assert.isAtLeast(
        filtered.length,
        1,
        "Should have at least 1 adoption with Serum program"
      );

      console.log(
        `\nFound ${filtered.length} adoption(s) including Serum program:`
      );
      filtered.forEach((adoption) => {
        const serumAccount = adoption.account.accounts.find(
          (account: any) => account.accountAddress === serumProgramAddress
        );
        console.log(`  PDA: ${adoption.publicKey.toBase58()}`);
        console.log(`  Serum contract scope:`, serumAccount?.childContractScope);
      });
    });

    it("Combined query: Fetch by adopter and filter by program", async () => {
      // This demonstrates the complete workflow:
      // 1. Fetch all adoptions by adopter (on-chain filter)
      // 2. Filter by program address (client-side filter)

      const adopter = owner.publicKey;
      const programAddress = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

      // Step 1: Fetch all adoptions by adopter
      const allAdoptions = await fetchAdoptionsByAdopter(adopter);
      console.log(`\nStep 1: Found ${allAdoptions.length} total adoptions for adopter`);

      // Step 2: Filter by program address
      const filteredAdoptions = filterAdoptionsByProgramAddress(
        allAdoptions,
        programAddress
      );
      console.log(
        `Step 2: Filtered to ${filteredAdoptions.length} adoptions containing ${programAddress}`
      );

      // Verify results
      assert.isAtLeast(allAdoptions.length, 1);
      assert.isAtLeast(filteredAdoptions.length, 1);
      assert.isAtMost(filteredAdoptions.length, allAdoptions.length);

      // Show summary
      console.log(`\nSummary:`);
      console.log(`  Total adoptions: ${allAdoptions.length}`);
      console.log(`  With ${programAddress}: ${filteredAdoptions.length}`);
    });
  });
});
