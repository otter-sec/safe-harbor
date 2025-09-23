import * as anchor from "@coral-xyz/anchor";
import { LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import * as fs from "fs";
const safeHarbourIDL = JSON.parse(
  fs.readFileSync("./target/idl/safe_harbour.json", "utf8")
);
import { Buffer } from "buffer";
import { BN } from "bn.js";

// Seeds
const AGREEMENT_SEED = Buffer.from("agreement_v2");
const ADOPTION_SEED = Buffer.from("adopt_v2");

async function main() {
  const provider = new anchor.AnchorProvider(
    new anchor.web3.Connection("http://127.0.0.1:8899"),
    anchor.Wallet.local(),
    { preflightCommitment: "confirmed" }
  );
  anchor.setProvider(provider);

  const owner = provider.wallet.payer;
  console.log(`Owner wallet public key: ${owner.publicKey.toBase58()}`);

  const wallet = provider.wallet.publicKey;
  const balance = await provider.connection.getBalance(wallet);
  console.log("Owner wallet balance:", balance / LAMPORTS_PER_SOL, "SOL");

  const program = new anchor.Program(safeHarbourIDL as anchor.Idl, provider);

  const initNonce = new BN(1);

  // Derive PDAs
  const [agreement] = PublicKey.findProgramAddressSync(
    [
      AGREEMENT_SEED,
      owner.publicKey.toBuffer(),
      Buffer.from(initNonce.toArrayLike(Buffer, "le", 8)),
    ],
    program.programId
  );

  const [adoption] = PublicKey.findProgramAddressSync(
    [ADOPTION_SEED, owner.publicKey.toBuffer()],
    program.programId
  );

  console.log(`Agreement PDA: ${agreement.toBase58()}`);
  console.log(`Adoption PDA: ${adoption.toBase58()}\n`);

  try {
    // Test 1: Create Agreement
    console.log("Test 1: Creating Agreement...");

    const agreementData = {
      protocolName: "Test Protocol Devnet",
      contactDetails: [{ name: "John Doe", contact: "john@example.com" }],
      bountyTerms: {
        bountyPercentage: new BN(10),
        bountyCapUsd: new BN(100000),
        retainable: false,
        identity: { anonymous: {} },
        diligenceRequirements: "Basic requirements",
        aggregateBountyCapUsd: new BN(0),
      },
      agreementUri: "ipfs://test-hash-devnet",
    };

    const createAgreementInst = await program.methods
      .createOrUpdateAgreement(
        initNonce,
        agreementData,
        owner.publicKey,
        0 // update_type: 0 = InitializeOrUpdate
      )
      .accounts({
        agreement: agreement,
        signer: owner.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log(
      "Agreement created successfully! Signature: ",
      createAgreementInst
    );

    // Test 2: Create Adoption
    console.log("Test 2: Creating Adoption...");

    const createAdoptionInst = await program.methods
      .createOrUpdateAdoption()
      .accounts({
        adoption: adoption,
        owner: owner.publicKey,
        agreement: agreement,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log(
      "Adoption created successfully! Signature: ",
      createAdoptionInst
    );

    console.log("Test 3: new Agreement");
    const newInitNonce = new BN(2);
    const newAgreement = PublicKey.findProgramAddressSync(
      [
        AGREEMENT_SEED,
        owner.publicKey.toBuffer(),
        Buffer.from(newInitNonce.toArrayLike(Buffer, "le", 8)),
      ],
      program.programId
    );

    const newAgreementData = {
      protocolName: "Test Protocol Devnet 2",
      contactDetails: [{ name: "Jane Doe", contact: "jane@example.com" }],
      bountyTerms: {
        bountyPercentage: new BN(15),
        bountyCapUsd: new BN(150000),
        retainable: true,
        identity: { anonymous: {} },
        diligenceRequirements: "Advanced requirements",
        aggregateBountyCapUsd: new BN(0),
      },
      agreementUri: "ipfs://test-hash-devnet-2",
    };

    const newCreateAgreementInst = await program.methods
      .createOrUpdateAgreement(
        newInitNonce,
        newAgreementData,
        owner.publicKey,
        0
      )
      .accounts({
        agreement: newAgreement,
        signer: owner.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log(
      "new Agreement created successfully! Signature: ",
      newCreateAgreementInst
    );

    // Test 4: update Adoption
    console.log("Test 4: Updating Adoption...");

    const updateAdoptionInst = await program.methods
      .createOrUpdateAdoption()
      .accounts({
        adoption: adoption,
        owner: owner.publicKey,
        agreement: agreement,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log(
      "Adoption updated successfully! Signature: ",
      updateAdoptionInst
    );
  } catch (error) {
    console.error("Test failed:", error);
    return false;
  }
}

main().then(() => process.exit());
