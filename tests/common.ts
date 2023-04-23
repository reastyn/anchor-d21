import { PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { Program } from "@project-serum/anchor";
import * as anchor from "@project-serum/anchor";
import { D21 } from "../target/types/d21";
import { expect } from "chai";

const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));

export const expectToThrow = async (
  fn: () => Promise<any>,
  errorCode: string
) => {
  try {
    await fn();
    expect.fail("function should have thrown");
  } catch (e) {
    expect(e.error.errorCode.code).to.equal(errorCode);
  }
};

export const getBasicInfoPDA = (
  program: Program<D21>
): ReturnType<typeof PublicKey.findProgramAddressSync> => {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("basic_info")],
    program.programId
  );
};

export const initialize = async (
  program: Program<D21>,
  owner: anchor.web3.Keypair
): Promise<[PublicKey, number]> => {
  const [basicInfoPDA, basicInfoBump] = getBasicInfoPDA(program);

  await program.methods
    .initialize(30)
    .accounts({ basicInfo: basicInfoPDA, initializer: owner.publicKey })
    .signers([owner])
    .rpc();

  return [basicInfoPDA, basicInfoBump];
};

export const blockRequestAirDrop = async (
  provider: anchor.Provider,
  publicKey: PublicKey
) => {
  let balance = await provider.connection.getBalance(publicKey);
  if (balance > 0) {
    return;
  }
  await provider.connection.requestAirdrop(publicKey, LAMPORTS_PER_SOL * 1);

  while (true) {
    balance = await provider.connection.getBalance(publicKey);

    if (balance > 0) {
      break;
    } else {
      await sleep(250);
    }
  }
};
