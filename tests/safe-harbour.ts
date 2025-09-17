import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SafeHarbour } from "../target/types/safe_harbour";
import { expect } from "chai";

// Utilities
const u64ToSeed = (value: number) => new anchor.BN(value).toArrayLike(Buffer, "be", 8);

describe("safe-harbour", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.SafeHarbour as unknown as Program<any>;
  const provider = anchor.getProvider() as anchor.AnchorProvider;
  const methods: any = (program as any).methods;
  const accountsNs: any = (program as any).account;

  // PDAs and signers
  let registry: anchor.web3.PublicKey;
  let owner: anchor.web3.Keypair;
  let agreement1: anchor.web3.PublicKey;
  let agreement2: anchor.web3.PublicKey;
  let adoption: anchor.web3.PublicKey;

  // Seeds
  const REGISTRY_SEED = Buffer.from("registry_v2");
  const AGREEMENT_SEED = Buffer.from("agreement_v2");
  const ADOPTION_SEED = Buffer.from("adoption_v2");

  // Common test agreement data builder
  const buildAgreementData = (protocolName: string, chainIds: string[]) => ({
    protocolName,
    contactDetails: [
      { name: "John Doe", contact: "john@example.com" },
    ],
    chains: chainIds.map((id) => ({
      assetRecoveryAddress: "0x1234567890123456789012345678901234567890",
      accounts: [
        {
          accountAddress: "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd",
          childContractScope: { none: {} },
        },
      ],
      caip2ChainId: id,
    })),
    bountyTerms: {
      bountyPercentage: new anchor.BN(10),
      bountyCapUsd: new anchor.BN(100000),
      retainable: false,
      identity: { anonymous: {} },
      diligenceRequirements: "Basic requirements",
      aggregateBountyCapUsd: new anchor.BN(0),
    },
    agreementUri: "ipfs://test-hash",
  });

  before(async () => {
    owner = anchor.web3.Keypair.generate();

    // Derive PDAs
    [registry] = anchor.web3.PublicKey.findProgramAddressSync(
      [REGISTRY_SEED],
      program.programId
    );

    const initNonce1 = 1;
    [agreement1] = anchor.web3.PublicKey.findProgramAddressSync(
      [AGREEMENT_SEED, owner.publicKey.toBuffer(), u64ToSeed(initNonce1)],
      program.programId
    );

    const initNonce2 = 2;
    [agreement2] = anchor.web3.PublicKey.findProgramAddressSync(
      [AGREEMENT_SEED, owner.publicKey.toBuffer(), u64ToSeed(initNonce2)],
      program.programId
    );

    [adoption] = anchor.web3.PublicKey.findProgramAddressSync(
      [ADOPTION_SEED, owner.publicKey.toBuffer()],
      program.programId
    );

    // Fund owner for transactions
    const sig = await provider.connection.requestAirdrop(
      owner.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(sig, "confirmed");
  });

  it("initializes registry, creates agreements, adopts and switches adoption", async () => {
    // 1) Initialize registry with valid chains
    const validChains = ["eip155:1", "eip155:11155111"]; // mainnet + sepolia
    await methods
      .initialize(validChains)
      .accounts({
        registry,
        signer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // 2) Create Agreement 1 for owner (InitializeOrUpdate)
    const data1 = buildAgreementData("Test Protocol A", ["eip155:1"]);
    await methods
      .createOrUpdateAgreement(
        data1,
        owner.publicKey,
        { initializeOrUpdate: {} },
        new anchor.BN(1)
      )
      .accounts({
        agreement: agreement1,
        registry,
        signer: owner.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([owner])
      .rpc();

    const agreementAccount1 = await accountsNs.agreementData.fetch(agreement1);
    expect(agreementAccount1.owner.toBase58()).to.equal(owner.publicKey.toBase58());
    expect(agreementAccount1.protocolName).to.equal("Test Protocol A");

    // 3) Create adoption pointing to agreement1
    await methods
      .createOrUpdateAdoption()
      .accounts({
        adoption,
        registry,
        owner: owner.publicKey,
        agreement: agreement1,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([owner])
      .rpc();

    const adoptionAccount1 = await accountsNs.adopt.fetch(adoption);
    expect(adoptionAccount1.agreement.toBase58()).to.equal(agreement1.toBase58());

    // 4) Create Agreement 2 for same owner
    const data2 = buildAgreementData("Test Protocol B", ["eip155:1"]);
    await methods
      .createOrUpdateAgreement(
        data2,
        owner.publicKey,
        { initializeOrUpdate: {} },
        new anchor.BN(2)
      )
      .accounts({
        agreement: agreement2,
        registry,
        signer: owner.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([owner])
      .rpc();

    const agreementAccount2 = await accountsNs.agreementData.fetch(agreement2);
    expect(agreementAccount2.owner.toBase58()).to.equal(owner.publicKey.toBase58());
    expect(agreementAccount2.protocolName).to.equal("Test Protocol B");

    // 5) Update adoption to point to agreement2
    await methods
      .createOrUpdateAdoption()
      .accounts({
        adoption,
        registry,
        owner: owner.publicKey,
        agreement: agreement2,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([owner])
      .rpc();

    const adoptionAccount2 = await accountsNs.adopt.fetch(adoption);
    expect(adoptionAccount2.agreement.toBase58()).to.equal(agreement2.toBase58());

    // 6) Verify registry mapping reflects the new agreement for owner
    const registryAccount = await accountsNs.registry.fetch(registry);
    const mapping = (registryAccount.agreements as { key: string; value: string }[])
      .find((e) => e.key === owner.publicKey.toBase58());
    expect(mapping).to.not.be.undefined;
    expect(mapping!.value).to.equal(agreement2.toBase58());
  });
});
