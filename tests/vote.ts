import { PublicKey } from "@solana/web3.js";
import { blockRequestAirDrop, expectToThrow, getBasicInfoPDA } from "./common";
import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { D21 } from "../target/types/d21";
import { expect } from "chai";
import { createVoter } from "./addVoter";
import { addSubject } from "./addSubject";

enum VoteType {
  Positive = 1,
  Negative = 0,
}

const vote = async (
  program: Program<D21>,
  subject: anchor.web3.Keypair,
  voter: anchor.web3.Keypair,
  positiveVote: VoteType
) => {
  const [subjectPDA, _subjectBump] = PublicKey.findProgramAddressSync(
    [Buffer.from("subject"), subject.publicKey.toBuffer()],
    program.programId
  );
  const [voterPDA, _voterBump] = PublicKey.findProgramAddressSync(
    [Buffer.from("voter"), voter.publicKey.toBuffer()],
    program.programId
  );
  const [basicInfo, _basicInfoBump] = getBasicInfoPDA(program);

  await program.methods
    .vote(subject.publicKey, positiveVote === 1)
    .accounts({
      voter: voterPDA,
      subject: subjectPDA,
      basicInfo: basicInfo,
      initializer: voter.publicKey,
    })
    .signers([voter])
    .rpc();
  const subjectAccount = await program.account.subjectAccount.fetch(subjectPDA);
  return subjectAccount;
};

export const testVote = (
  program: Program<D21>,
  owner: anchor.web3.Keypair,
  provider: anchor.AnchorProvider
) => {
  const voter = anchor.web3.Keypair.generate();
  const subject = anchor.web3.Keypair.generate();
  const subject2 = anchor.web3.Keypair.generate();
  const subject3 = anchor.web3.Keypair.generate();
  const subject4 = anchor.web3.Keypair.generate();

  it("should not be able to vote negatively before positive votes", async () => {
    await createVoter(program, owner, voter);
    await blockRequestAirDrop(provider, subject.publicKey);
    await addSubject(program, subject);

    await expectToThrow(
      async () => await vote(program, subject, voter, VoteType.Negative),
      "NegativeVotesAfterTwoPositive"
    );
  });

  it("should be able to vote", async () => {
    const subjectAccount = await vote(
      program,
      subject,
      voter,
      VoteType.Positive
    );
    expect(subjectAccount.votes.toNumber()).to.be.eq(1);
  });

  it("should not be possible to vote twice for same person", async () => {
    await expectToThrow(
      async () => await vote(program, subject, voter, VoteType.Positive),
      "VoteForSameSubjectTwice"
    );
  });

  it("should be able to vote for second time", async () => {
    await blockRequestAirDrop(provider, subject2.publicKey);
    await addSubject(program, subject2);

    const subjectAccount = await vote(
      program,
      subject2,
      voter,
      VoteType.Positive
    );
    expect(subjectAccount.votes.toNumber()).to.be.eq(1);
  });

  it("should not be able to vote for third time", async () => {
    await blockRequestAirDrop(provider, subject3.publicKey);
    await addSubject(program, subject3);

    await expectToThrow(
      async () => await vote(program, subject3, voter, VoteType.Positive),
      "NoMorePositiveVotes"
    );
  });

  it("should not be able to vote negatively", async () => {
    const subjectAccount = await vote(
      program,
      subject3,
      voter,
      VoteType.Negative
    );
    expect(subjectAccount.votes.toNumber()).to.be.eq(-1);
  });

  it("should not be able to vote negatively twice", async () => {
    await blockRequestAirDrop(provider, subject4.publicKey);
    await addSubject(program, subject4);

    await expectToThrow(
      async () => await vote(program, subject4, voter, VoteType.Negative),
      "NoMoreNegativeVotes"
    );
  });
};
