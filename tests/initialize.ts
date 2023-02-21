import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { D21 } from "../target/types/d21";
import { expect } from "chai";
import { initialize } from "./common";

export const testInitialize = (program: Program<D21>, owner: anchor.web3.Keypair) => {
  it("should initialize", async () => {
    const [basicInfoPDA] = await initialize(program, owner);
    const account = await program.account.basicInfo.fetch(basicInfoPDA);
    expect(account.owner.toBase58()).to.be.eq(owner.publicKey.toBase58());
    expect(account.endDate).to.be.ok;
  });

  it("should fail to initialize twice", async () => {
    expect(() => initialize(program, owner)).to.throw;
  });
};
