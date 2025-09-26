import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram, Keypair } from "@solana/web3.js";
import { Buffer } from "buffer";
import { BN } from "bn.js";
import { assert } from "chai";
import { SafeHarbor } from "../../target/types/safe_harbor";

// Seeds
const AGREEMENT_SEED = Buffer.from("agreement_v2");
const ADOPTION_SEED = Buffer.from("adopt_v2");
const REGISTRY_SEED = Buffer.from("registry_v2");

describe("Adoption Tests", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SafeHarbor as Program<SafeHarbor>;
  const provider = anchor.getProvider();
  const owner = (provider.wallet as anchor.Wallet).payer;
  let secondUser: Keypair;
  let adoption_pda: PublicKey;
  let registry_pda: PublicKey;

  before(async () => {
    // Create a second user for testing
    secondUser = Keypair.generate();

    // Derive PDAs
    [adoption_pda] = PublicKey.findProgramAddressSync(
      [ADOPTION_SEED, owner.publicKey.toBuffer()],
      program.programId,
    );

    [registry_pda] = PublicKey.findProgramAddressSync(
      [REGISTRY_SEED],
      program.programId,
    );

    console.log(`Adoption PDA: ${adoption_pda.toBase58()}`);
    console.log(`Registry PDA: ${registry_pda.toBase58()}`);
  });

  it("Create New Adoption", async () => {
    // Generate agreement PDA for owner
    const agreementPda = PublicKey.findProgramAddressSync(
      [
        AGREEMENT_SEED,
        owner.publicKey.toBuffer(),
        Buffer.from(new BN(1).toArrayLike(Buffer, "le", 8)),
      ],
      program.programId,
    )[0];

    const tx = await program.methods
      .createOrUpdateAdoption()
      .accounts({
        adoption: adoption_pda,
        owner: owner.publicKey,
        agreement: agreementPda,
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

    // Fetch and verify the created adoption
    const adoptionAccount = await program.account.adopt.fetch(adoption_pda);

    // Assert adoption data
    assert.equal(adoptionAccount.agreement.toString(), agreementPda.toString());
  });

  it("Update Adoption", async () => {
    // Generate a different agreement PDA for the update
    const newAgreementPda = PublicKey.findProgramAddressSync(
      [
        AGREEMENT_SEED,
        owner.publicKey.toBuffer(),
        Buffer.from(new BN(2).toArrayLike(Buffer, "le", 8)),
      ],
      program.programId,
    )[0];

    // Now update the adoption to point to the new agreement
    const updateTx = await program.methods
      .createOrUpdateAdoption()
      .accounts({
        adoption: adoption_pda,
        owner: owner.publicKey,
        agreement: newAgreementPda,
        systemProgram: SystemProgram.programId,
      } as any)
      .rpc();

    // Wait for confirmation
    const updateBlockHash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      blockhash: updateBlockHash.blockhash,
      lastValidBlockHeight: updateBlockHash.lastValidBlockHeight,
      signature: updateTx,
    });

    // Fetch and verify the updated adoption
    const adoptionAccount = await program.account.adopt.fetch(adoption_pda);

    assert.equal(
      adoptionAccount.agreement.toString(),
      newAgreementPda.toString(),
    );
  });

  it("Non-owner tries to update adoption - should fail", async () => {
    // Airdrop SOL to second user for gas
    const airdropTx = await provider.connection.requestAirdrop(
      secondUser.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL,
    );
    await provider.connection.confirmTransaction(airdropTx);

    // Generate agreement PDA for second user
    const secondUserAgreementPda = PublicKey.findProgramAddressSync(
      [
        AGREEMENT_SEED,
        secondUser.publicKey.toBuffer(),
        Buffer.from(new BN(1).toArrayLike(Buffer, "le", 8)),
      ],
      program.programId,
    )[0];

    try {
      await program.methods
        .createOrUpdateAdoption()
        .accounts({
          adoption: adoption_pda, // Trying to update owner's adoption
          owner: secondUser.publicKey, // But signing as second user
          agreement: secondUserAgreementPda,
          systemProgram: SystemProgram.programId,
        } as any)
        .signers([secondUser])
        .rpc();
      assert.fail(
        "Expected transaction to fail - non-owner cannot update adoption",
      );
    } catch (error) {
      // The transaction should fail because second user is not the owner
    }
  });
});
