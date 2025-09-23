import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { Buffer } from "buffer";
import { BN } from "bn.js";
import { assert } from "chai";
import { SafeHarbour } from "../target/types/safe_harbour";

// Seeds
const AGREEMENT_SEED = Buffer.from("agreement_v2");
const ADOPTION_SEED = Buffer.from("adopt_v2");

describe("safe_harbour v2", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SafeHarbour as Program<SafeHarbour>;
  const provider = anchor.getProvider();
  const owner = (provider.wallet as anchor.Wallet).payer;

  let agreement_pda: PublicKey;
  let adoption_pda: PublicKey;

  before(async () => {
    const initNonce = new BN(1);

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
      [ADOPTION_SEED, owner.publicKey.toBuffer()],
      program.programId
    );

    console.log(`Agreement PDA: ${agreement_pda.toBase58()}`);
    console.log(`Adoption PDA: ${adoption_pda.toBase58()}`);
  });

  it("Create Agreement", async () => {
    const initNonce = new BN(1);
    const agreementData = {
      owner: owner.publicKey,
      protocolName: "Safe Harbour Protocol",
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
    assert.equal(agreementAccount.protocolName, "Safe Harbour Protocol");
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
    const tx = await program.methods
      .createOrUpdateAdoption()
      .accounts({
        adoption: adoption_pda,
        owner: owner.publicKey,
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
  });

  it("Update Agreement", async () => {
    const initNonce = new BN(1);
    const updatedAgreementData = {
      owner: owner.publicKey,
      protocolName: "Safe Harbour Protocol v2",
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
      "Safe Harbour Protocol v2"
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
