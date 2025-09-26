import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram, Keypair } from "@solana/web3.js";
import { Buffer } from "buffer";
import { BN } from "bn.js";
import { assert, expect } from "chai";
import { SafeHarbor } from "../../target/types/safe_harbor";

// Seeds
const REGISTRY_SEED = Buffer.from("registry_v2");

describe("Registry Tests", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SafeHarbor as Program<SafeHarbor>;
  const provider = anchor.getProvider();
  const owner = (provider.wallet as anchor.Wallet).payer;
  let secondUser: Keypair;
  let registry_pda: PublicKey;

  before(async () => {
    // Create a second user for testing
    secondUser = Keypair.generate();

    // Derive registry PDA
    [registry_pda] = PublicKey.findProgramAddressSync(
      [REGISTRY_SEED],
      program.programId,
    );

    console.log(`Registry PDA: ${registry_pda.toBase58()}`);
  });

  it("Initialize Registry", async () => {
    const validChains = [new BN(1), new BN(137), new BN(42161)];

    const tx = await program.methods
      .initialize(validChains)
      .accounts({
        registry: registry_pda,
        signer: owner.publicKey,
        systemProgram: SystemProgram.programId,
      } as any)
      .rpc();

    // Wait for confirmation
    const latestBlockHash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: tx,
    });

    // Fetch and verify the registry
    const registryAccount = await program.account.registry.fetch(registry_pda);

    assert.equal(registryAccount.owner.toString(), owner.publicKey.toString());
    assert.equal(registryAccount.initialized, true);
    expect(registryAccount.validChains.map((c) => c.toNumber())).to.deep.equal([
      1, 137, 42161,
    ]);
  });

  it("Set Valid Chains", async () => {
    const newValidChains = [new BN(56), new BN(250)];

    const tx = await program.methods
      .setValidChains(newValidChains)
      .accounts({
        registry: registry_pda,
        signer: owner.publicKey,
        systemProgram: SystemProgram.programId,
      } as any)
      .rpc();

    // Wait for confirmation
    const latestBlockHash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: tx,
    });

    // Fetch and verify the updated registry
    const registryAccount = await program.account.registry.fetch(registry_pda);
    assert.isTrue(registryAccount.validChains.some((c) => c.eq(new BN(56))));
    assert.isTrue(registryAccount.validChains.some((c) => c.eq(new BN(250))));
  });

  it("Set Invalid Chains", async () => {
    const invalidChains = [new BN(56)]; // Remove BSC

    const tx = await program.methods
      .setInvalidChains(invalidChains)
      .accounts({
        registry: registry_pda,
        signer: owner.publicKey,
        systemProgram: SystemProgram.programId,
      } as any)
      .rpc();

    // Wait for confirmation
    const latestBlockHash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: tx,
    });

    // Fetch and verify the updated registry
    const registryAccount = await program.account.registry.fetch(registry_pda);
    assert.isFalse(registryAccount.validChains.some((c) => c.eq(new BN(56))));
    assert.isTrue(registryAccount.validChains.some((c) => c.eq(new BN(250)))); // Fantom should still be there
  });

  it("Should fail when non-owner tries to set valid chains", async () => {
    const newValidChains = [new BN(10), new BN(56)]; // Optimism, BSC

    try {
      await program.methods
        .setValidChains(newValidChains)
        .accounts({
          registry: registry_pda,
          signer: secondUser.publicKey, // Non-owner
          systemProgram: SystemProgram.programId,
        } as any)
        .signers([secondUser])
        .rpc();
      assert.fail(
        "Expected transaction to fail when non-owner tries to set valid chains",
      );
    } catch (error) {
      assert.include(error.message, "NotRegistryOwner");
    }
  });

  it("Should fail when non-owner tries to set invalid chains", async () => {
    const invalidChains = [new BN(250)]; // Remove Fantom

    try {
      await program.methods
        .setInvalidChains(invalidChains)
        .accounts({
          registry: registry_pda,
          signer: secondUser.publicKey, // Non-owner
          systemProgram: SystemProgram.programId,
        } as any)
        .signers([secondUser])
        .rpc();
      assert.fail(
        "Expected transaction to fail when non-owner tries to set invalid chains",
      );
    } catch (error) {
      assert.include(error.message, "NotRegistryOwner");
    }
  });
});
