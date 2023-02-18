import { PublicKey } from "@solana/web3.js";
import { blockRequestAirDrop, expectToThrow, getBasicInfoPDA } from "./common";
import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { D21 } from "../target/types/d21";
import { expect } from "chai";

export const createVoter = async (
  program: Program<D21>,
  owner: anchor.web3.Keypair,
  newVoter: anchor.web3.Keypair
): Promise<[PublicKey, number]> => {
  const [voterPDA, voterBump] = PublicKey.findProgramAddressSync(
    [Buffer.from("voter"), newVoter.publicKey.toBuffer()],
    program.programId
  );
  const [basicInfoPDA, bump] = getBasicInfoPDA(program);

  await program.methods
    .addVoter(bump, newVoter.publicKey)
    .accounts({
      voter: voterPDA,
      initializer: owner.publicKey,
      basicInfo: basicInfoPDA,
    })
    .signers([owner])
    .rpc();
  return [voterPDA, voterBump];
};

export const testAddVoter = (
  program: Program<D21>,
  owner: anchor.web3.Keypair,
  provider: anchor.AnchorProvider
) => {
  const newVoter = anchor.web3.Keypair.generate();

  it("should not be able to add voter if not owner", async () => {
    await blockRequestAirDrop(provider, newVoter.publicKey);
    await expectToThrow(async () => createVoter(program, newVoter, newVoter), "NotOwner");
  });

  it("owner should add voter", async () => {
    const [voterPDA] = await createVoter(program, owner, newVoter);
    const account = await program.account.voterAccount.fetch(voterPDA);
    expect(account.isInitialized).to.be.true;
  });

  it("should not be able to add voter multiple times", async () => {
    expect(createVoter(program, owner, newVoter)).to.throw;
  });
};
