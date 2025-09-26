import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram, Keypair } from "@solana/web3.js";
import { Buffer } from "buffer";
import { BN } from "bn.js";
import { assert } from "chai";
import { SafeHarbor } from "../../target/types/safe_harbor";

// Seeds
const AGREEMENT_SEED = Buffer.from("agreement_v2");
const REGISTRY_SEED = Buffer.from("registry_v2");

describe("Agreement Tests", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SafeHarbor as Program<SafeHarbor>;
  const provider = anchor.getProvider();
  const owner = (provider.wallet as anchor.Wallet).payer;
  let secondUser: Keypair;
  let agreement_pda: PublicKey;
  let registry_pda: PublicKey;
  let secondAgreement_pda: PublicKey;

  before(async () => {
    const initNonce = new BN(1);
    const secondInitNonce = new BN(2);

    // Create a second user for testing
    secondUser = Keypair.generate();

    // Derive PDAs
    [agreement_pda] = PublicKey.findProgramAddressSync(
      [
        AGREEMENT_SEED,
        owner.publicKey.toBuffer(),
        Buffer.from(initNonce.toArrayLike(Buffer, "le", 8)),
      ],
      program.programId,
    );

    [secondAgreement_pda] = PublicKey.findProgramAddressSync(
      [
        AGREEMENT_SEED,
        owner.publicKey.toBuffer(),
        Buffer.from(secondInitNonce.toArrayLike(Buffer, "le", 8)),
      ],
      program.programId,
    );

    [registry_pda] = PublicKey.findProgramAddressSync(
      [REGISTRY_SEED],
      program.programId,
    );

    console.log(`Agreement PDA: ${agreement_pda.toBase58()}`);
    console.log(`Registry PDA: ${registry_pda.toBase58()}`);
  });

  it("Create Agreement", async () => {
    const initNonce = new BN(1);
    const agreementData = {
      owner: owner.publicKey,
      protocolName: "Safe harbor Protocol",
      contactDetails: [
        { name: "Security Team", contact: "security@example.com" },
      ],
      chains: [
        {
          caip2ChainId: new BN(1),
          assetRecoveryAddress: owner.publicKey,
          accounts: [
            {
              accountAddress: owner.publicKey,
              childContractScope: { existingOnly: {} },
            },
          ],
        },
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

    // Fetch and verify the created agreement
    const agreementAccount =
      await program.account.agreementData.fetch(agreement_pda);

    // Assert agreement data
    assert.equal(agreementAccount.protocolName, "Safe harbor Protocol");
    assert.equal(
      agreementAccount.bountyTerms.bountyPercentage.toString(),
      "10",
    );
    assert.equal(
      agreementAccount.bountyTerms.bountyCapUsd.toString(),
      "100000",
    );
    assert.equal(agreementAccount.owner.toString(), owner.publicKey.toString());
    assert.equal(agreementAccount.chains.length, 1);
    assert.equal(agreementAccount.chains[0].caip2ChainId.toString(), "1");
  });

  it("Create Agreement For Different Nonce", async () => {
    const initNonce = new BN(2);
    const agreementData = {
      owner: owner.publicKey,
      protocolName: "Safe harbor Protocol",
      contactDetails: [
        { name: "Security Team", contact: "security@example.com" },
      ],
      chains: [
        {
          caip2ChainId: new BN(1),
          assetRecoveryAddress: owner.publicKey,
          accounts: [
            {
              accountAddress: owner.publicKey,
              childContractScope: { existingOnly: {} },
            },
          ],
        },
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
        agreement: secondAgreement_pda,
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

    // Fetch and verify the created agreement
    const agreementAccount =
      await program.account.agreementData.fetch(agreement_pda);

    // Assert agreement data
    assert.equal(agreementAccount.protocolName, "Safe harbor Protocol");
    assert.equal(
      agreementAccount.bountyTerms.bountyPercentage.toString(),
      "10",
    );
    assert.equal(
      agreementAccount.bountyTerms.bountyCapUsd.toString(),
      "100000",
    );
    assert.equal(agreementAccount.owner.toString(), owner.publicKey.toString());
    assert.equal(agreementAccount.chains.length, 1);
    assert.equal(agreementAccount.chains[0].caip2ChainId.toString(), "1");
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
      chains: [
        {
          caip2ChainId: new BN(1),
          assetRecoveryAddress: owner.publicKey,
          accounts: [
            {
              accountAddress: owner.publicKey,
              childContractScope: { existingOnly: {} },
            },
          ],
        },
        {
          caip2ChainId: new BN(137),
          assetRecoveryAddress: owner.publicKey,
          accounts: [
            {
              accountAddress: owner.publicKey,
              childContractScope: { all: {} },
            },
          ],
        },
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
        0, // update_type: 0 = InitializeOrUpdate
      )
      .accounts({
        agreement: agreement_pda,
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

    // Fetch and verify the updated agreement
    const updatedAgreementAccount =
      await program.account.agreementData.fetch(agreement_pda);

    // Assert updated agreement data
    assert.equal(
      updatedAgreementAccount.protocolName,
      "Safe harbor Protocol v2",
    );
    assert.equal(
      updatedAgreementAccount.bountyTerms.bountyPercentage.toString(),
      "15",
    );
    assert.equal(
      updatedAgreementAccount.bountyTerms.bountyCapUsd.toString(),
      "250000",
    );
    assert.equal(updatedAgreementAccount.bountyTerms.retainable, true);
    assert.equal(updatedAgreementAccount.contactDetails.length, 2);
    assert.equal(updatedAgreementAccount.chains.length, 2);
  });

  it("Update Agreement Protocol Name Only", async () => {
    const initNonce = new BN(1);
    const protocolNameData = {
      owner: owner.publicKey,
      protocolName: "Updated Protocol Name Only",
      contactDetails: [],
      chains: [],
      bountyTerms: {
        bountyPercentage: new BN(0),
        bountyCapUsd: new BN(0),
        retainable: false,
        identity: { anonymous: {} },
        diligenceRequirements: "",
        aggregateBountyCapUsd: new BN(0),
      },
      agreementUri: "",
    };

    const tx = await program.methods
      .createOrUpdateAgreement(
        initNonce,
        protocolNameData,
        owner.publicKey,
        1, // update_type: 1 = ProtocolName
      )
      .accounts({
        agreement: agreement_pda,
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

    // Fetch and verify the updated agreement
    const updatedAgreementAccount =
      await program.account.agreementData.fetch(agreement_pda);

    // Only protocol name should be updated
    assert.equal(
      updatedAgreementAccount.protocolName,
      "Updated Protocol Name Only",
    );
    // Other fields should remain unchanged
    assert.equal(updatedAgreementAccount.contactDetails.length, 2);
    assert.equal(updatedAgreementAccount.chains.length, 2);
  });

  it("Update Agreement Contact Details Only", async () => {
    const initNonce = new BN(1);
    const contactDetailsData = {
      owner: owner.publicKey,
      protocolName: "",
      contactDetails: [
        { name: "New Contact", contact: "new@example.com" },
        { name: "Emergency Contact", contact: "emergency@example.com" },
        { name: "Legal Team", contact: "legal@example.com" },
      ],
      chains: [],
      bountyTerms: {
        bountyPercentage: new BN(0),
        bountyCapUsd: new BN(0),
        retainable: false,
        identity: { anonymous: {} },
        diligenceRequirements: "",
        aggregateBountyCapUsd: new BN(0),
      },
      agreementUri: "",
    };

    const tx = await program.methods
      .createOrUpdateAgreement(
        initNonce,
        contactDetailsData,
        owner.publicKey,
        2, // update_type: 2 = ContactDetails
      )
      .accounts({
        agreement: agreement_pda,
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

    // Fetch and verify the updated agreement
    const updatedAgreementAccount =
      await program.account.agreementData.fetch(agreement_pda);

    // Only contact details should be updated
    assert.equal(updatedAgreementAccount.contactDetails.length, 3);
    assert.equal(updatedAgreementAccount.contactDetails[0].name, "New Contact");
    assert.equal(
      updatedAgreementAccount.contactDetails[0].contact,
      "new@example.com",
    );
    // Other fields should remain unchanged
    assert.equal(
      updatedAgreementAccount.protocolName,
      "Updated Protocol Name Only",
    );
  });

  it("Update Agreement Bounty Terms Only", async () => {
    const initNonce = new BN(1);
    const bountyTermsData = {
      owner: owner.publicKey,
      protocolName: "",
      contactDetails: [],
      chains: [],
      bountyTerms: {
        bountyPercentage: new BN(25),
        bountyCapUsd: new BN(500000),
        retainable: true,
        identity: { pseudonymous: {} },
        diligenceRequirements: "Enhanced security requirements",
        aggregateBountyCapUsd: new BN(0),
      },
      agreementUri: "",
    };

    const tx = await program.methods
      .createOrUpdateAgreement(
        initNonce,
        bountyTermsData,
        owner.publicKey,
        3, // update_type: 3 = BountyTerms
      )
      .accounts({
        agreement: agreement_pda,
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

    // Fetch and verify the updated agreement
    const updatedAgreementAccount =
      await program.account.agreementData.fetch(agreement_pda);

    // Only bounty terms should be updated
    assert.equal(
      updatedAgreementAccount.bountyTerms.bountyPercentage.toString(),
      "25",
    );
    assert.equal(
      updatedAgreementAccount.bountyTerms.bountyCapUsd.toString(),
      "500000",
    );
    assert.equal(updatedAgreementAccount.bountyTerms.retainable, true);
    assert.equal(
      updatedAgreementAccount.bountyTerms.diligenceRequirements,
      "Enhanced security requirements",
    );
    // Other fields should remain unchanged
    assert.equal(
      updatedAgreementAccount.protocolName,
      "Updated Protocol Name Only",
    );
  });

  it("Update Agreement URI Only", async () => {
    const initNonce = new BN(1);
    const agreementUriData = {
      owner: owner.publicKey,
      protocolName: "",
      contactDetails: [],
      chains: [],
      bountyTerms: {
        bountyPercentage: new BN(0),
        bountyCapUsd: new BN(0),
        retainable: false,
        identity: { anonymous: {} },
        diligenceRequirements: "",
        aggregateBountyCapUsd: new BN(0),
      },
      agreementUri: "ipfs://QmNewAgreementHash",
    };

    const tx = await program.methods
      .createOrUpdateAgreement(
        initNonce,
        agreementUriData,
        owner.publicKey,
        4, // update_type: 4 = AgreementUri
      )
      .accounts({
        agreement: agreement_pda,
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

    // Fetch and verify the updated agreement
    const updatedAgreementAccount =
      await program.account.agreementData.fetch(agreement_pda);

    // Only agreement URI should be updated
    assert.equal(
      updatedAgreementAccount.agreementUri,
      "ipfs://QmNewAgreementHash",
    );
    // Other fields should remain unchanged
    assert.equal(
      updatedAgreementAccount.protocolName,
      "Updated Protocol Name Only",
    );
  });

  it("Update Agreement Chains Only", async () => {
    const initNonce = new BN(1);
    const chainsData = {
      owner: owner.publicKey,
      protocolName: "",
      contactDetails: [],
      chains: [
        {
          caip2ChainId: new BN(42161), // Arbitrum
          assetRecoveryAddress: owner.publicKey,
          accounts: [
            {
              accountAddress: owner.publicKey,
              childContractScope: { futureOnly: {} },
            },
          ],
        },
      ],
      bountyTerms: {
        bountyPercentage: new BN(0),
        bountyCapUsd: new BN(0),
        retainable: false,
        identity: { anonymous: {} },
        diligenceRequirements: "",
        aggregateBountyCapUsd: new BN(0),
      },
      agreementUri: "",
    };

    const tx = await program.methods
      .createOrUpdateAgreement(
        initNonce,
        chainsData,
        owner.publicKey,
        5, // update_type: 5 = Chains
      )
      .accounts({
        agreement: agreement_pda,
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

    // Fetch and verify the updated agreement
    const updatedAgreementAccount =
      await program.account.agreementData.fetch(agreement_pda);

    // Only chains should be updated
    assert.equal(updatedAgreementAccount.chains.length, 1);
    assert.equal(
      updatedAgreementAccount.chains[0].caip2ChainId.toString(),
      "42161",
    );
    // Other fields should remain unchanged
    assert.equal(
      updatedAgreementAccount.protocolName,
      "Updated Protocol Name Only",
    );
  });

  it("Create Second Agreement for Different User", async () => {
    // Airdrop SOL to second user for gas
    const airdropTx = await provider.connection.requestAirdrop(
      secondUser.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL, // 2 SOL for gas
    );
    await provider.connection.confirmTransaction(airdropTx);

    const secondUserAgreementPda = PublicKey.findProgramAddressSync(
      [
        AGREEMENT_SEED,
        secondUser.publicKey.toBuffer(),
        Buffer.from(new BN(1).toArrayLike(Buffer, "le", 8)),
      ],
      program.programId,
    )[0];

    const agreementData = {
      owner: secondUser.publicKey,
      protocolName: "Second User Protocol",
      contactDetails: [
        { name: "Second User Team", contact: "second@example.com" },
      ],
      chains: [
        {
          caip2ChainId: new BN(250), // Fantom
          assetRecoveryAddress: secondUser.publicKey,
          accounts: [
            {
              accountAddress: secondUser.publicKey,
              childContractScope: { none: {} },
            },
          ],
        },
      ],
      bountyTerms: {
        bountyPercentage: new BN(20),
        bountyCapUsd: new BN(200000),
        retainable: true,
        identity: { named: {} },
        diligenceRequirements: "Full KYC required",
        aggregateBountyCapUsd: new BN(0),
      },
      agreementUri: "ipfs://QmSecondUserAgreement",
    };

    const tx = await program.methods
      .createOrUpdateAgreement(
        new BN(1),
        agreementData,
        secondUser.publicKey,
        0,
      )
      .accounts({
        agreement: secondUserAgreementPda,
        registry: registry_pda,
        signer: secondUser.publicKey,
        systemProgram: SystemProgram.programId,
      } as any)
      .signers([secondUser])
      .rpc();

    // Wait for confirmation
    const latestBlockHash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: tx,
    });

    // Fetch and verify the created agreement
    const agreementAccount = await program.account.agreementData.fetch(
      secondUserAgreementPda,
    );

    assert.equal(agreementAccount.protocolName, "Second User Protocol");
    assert.equal(
      agreementAccount.owner.toString(),
      secondUser.publicKey.toString(),
    );
    assert.equal(agreementAccount.chains[0].caip2ChainId.toString(), "250");
  });
});
