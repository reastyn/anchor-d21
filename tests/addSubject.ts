import { PublicKey } from "@solana/web3.js";
import { expect } from "chai";
import { blockRequestAirDrop, getBasicInfoPDA } from "./common";
import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { D21 } from "../target/types/d21";

export const addSubject = async (
  program: Program<D21>,
  newSubject: anchor.web3.Keypair
): Promise<[PublicKey, number]> => {
  const [subjectPDA, subjectSeed] = PublicKey.findProgramAddressSync(
    [Buffer.from("subject"), newSubject.publicKey.toBuffer()],
    program.programId
  );
  const [basicInfoPDA, _bump] = getBasicInfoPDA(program);

  await program.methods
    .addSubject("Test subject :)")
    .accounts({
      subject: subjectPDA,
      initializer: newSubject.publicKey,
      basicInfo: basicInfoPDA,
    })
    .signers([newSubject])
    .rpc();
  return [subjectPDA, subjectSeed];
};

export const testAddSubject = (
  program: Program<D21>,
  provider: anchor.AnchorProvider
) => {
  it("should add subject", async () => {
    const newSubject = anchor.web3.Keypair.generate();
    await blockRequestAirDrop(provider, newSubject.publicKey);
    const [subjectPDA, _] = await addSubject(program, newSubject);

    const account = await program.account.subjectAccount.fetch(subjectPDA);
    expect(account.name).to.be.eq("Test subject :)");
    expect(account.votes.toNumber()).to.be.eq(0);
  });
};
