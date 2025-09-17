import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SafeHarbour } from "../target/types/safe_harbour";
import { expect } from "chai";

describe("safe-harbour", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.SafeHarbour as Program<SafeHarbour>;
  const provider = anchor.getProvider();

  let registry: anchor.web3.PublicKey;
  let agreement: anchor.web3.Keypair;
  let adoption: anchor.web3.PublicKey;
  let owner: anchor.web3.Keypair;
  let adopter: anchor.web3.Keypair;

  const testAgreementData = {
    protocolName: "Test Protocol",
    contactDetails: [
      {
        name: "John Doe",
        contact: "john@example.com"
      }
    ],
    chains: [
      {
        assetRecoveryAddress: "0x1234567890123456789012345678901234567890",
        accounts: [
          {
            accountAddress: "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd",
            childContractScope: { none: {} }
          }
        ],
        caip2ChainId: "eip155:1"
      }
    ],
    bountyTerms: {
      bountyPercentage: new anchor.BN(10),
      bountyCapUsd: new anchor.BN(100000),
      retainable: false,
      identity: { anonymous: {} },
      diligenceRequirements: "Basic requirements",
      aggregateBountyCapUsd: new anchor.BN(0)
    },
    agreementUri: "ipfs://test-hash"
  };

  before(async () => {
    owner = anchor.web3.Keypair.generate();
    adopter = anchor.web3.Keypair.generate();
    agreement = anchor.web3.Keypair.generate();

    [registry] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("registry_v2")],
      program.programId
    );

    [adoption] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("adoption_v2"), adopter.publicKey.toBuffer(), agreement.publicKey.toBuffer()],
      program.programId
    );
  });

  it("Creates an agreement", async () => {
    await program.methods
      .createOrUpdateAgreement(testAgreementData, owner.publicKey)
      .accounts({
        registry: registry,
        agreement: agreement.publicKey,
        owner: owner.publicKey,
        payer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([agreement])
      .rpc();

    const agreementAccount = await program.account.agreement.fetch(agreement.publicKey);
    expect(agreementAccount.owner.toString()).to.equal(owner.publicKey.toString());
    expect(agreementAccount.data.protocolName).to.equal(testAgreementData.protocolName);
  });

  it("Creates an adoption entry", async () => {
    await program.methods
      .createOrUpdateAdoption()
      .accounts({
        registry: registry,
        adopter: adopter.publicKey,
        adoption: adoption,
        agreement: agreement.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([adopter])
      .rpc();

    const adoptionAccount = await program.account.adopt.fetch(adoption);
    expect(adoptionAccount.agreement.toString()).to.equal(agreement.publicKey.toString());
  });
});
