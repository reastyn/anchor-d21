import { Program } from "@project-serum/anchor";
import { D21 } from "../target/types/d21";
import { blockRequestAirDrop } from "./common";
import { testInitialize } from "./initialize";
import * as anchor from "@project-serum/anchor";
import { testAddSubject } from "./addSubject";
import { testAddVoter } from "./addVoter";
import { testVote } from "./vote";

describe("d21", () => {
  const program = anchor.workspace.D21 as Program<D21>;
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const owner = anchor.web3.Keypair.generate();

  before(async () => {
    await blockRequestAirDrop(provider, owner.publicKey);
  });

  describe("initialize", () => {
    testInitialize(program, owner);
  });

  describe("add subject", () => {
    testAddSubject(program, provider);
  });

  describe("get subjects", () => {
    it("should return all subjects", async () => {
      const subjectAccounts = await program.account.subjectAccount.all();
      console.log(subjectAccounts);
    });
  });

  describe("owner should be able to add voter", () => {
    testAddVoter(program, owner, provider);
  });

  describe("voter should be able to vote", () => {
    testVote(program, owner, provider);
  });
});
