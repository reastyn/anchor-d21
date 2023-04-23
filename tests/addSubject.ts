import { PublicKey } from "@solana/web3.js";
import { expect } from "chai";
import { blockRequestAirDrop, getBasicInfoPDA } from "./common";
import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { D21 } from "../target/types/d21";

export const addSubject = async (
  program: Program<D21>,
  newSubject: anchor.web3.Keypair,
  name: string = "Test subject :)"
): Promise<[PublicKey, number]> => {
  const [subjectPDA, subjectSeed] = PublicKey.findProgramAddressSync(
    [Buffer.from("subject"), newSubject.publicKey.toBuffer()],
    program.programId
  );
  const [basicInfoPDA, _bump] = getBasicInfoPDA(program);

  await program.methods
    .addSubject(name)
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
  let subject = null;
  it("should add subject", async () => {
    subject = anchor.web3.Keypair.generate();
    await blockRequestAirDrop(provider, subject.publicKey);
    const [subjectPDA, _] = await addSubject(
      program,
      subject,
      "Test subject :)",
    );

    const account = await program.account.subjectAccount.fetch(subjectPDA);
    expect(account.name).to.be.eq("Test subject :)");
    expect(account.votes.toNumber()).to.be.eq(0);
  });

  it("should not add subject twice", async () => {
    expect(addSubject(program, subject, "test")).to.throw;
  });

  it("should not add subject with longer name", async () => {
    expect(
      addSubject(
        program,
        subject,
        "A name that is longer than 64 characters should not be accepted :)",
      )
    ).to.throw;
  });
};
