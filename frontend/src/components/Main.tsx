import { AnchorProvider, Program } from "@project-serum/anchor";
import { useConnection, useWallet } from "@solana/wallet-adapter-react";
// import idl from "./../../../target/idl/d21.json"; // Just so you don't have to build it :)
import idl from "./d21.json";
import { D21 } from "./d21";
import { PublicKey } from "@solana/web3.js";

const idlString = JSON.stringify(idl);
const idlObject = JSON.parse(idlString);
const programId = idl.metadata.address;

export const getBasicInfoPDA = (
  program: Program<D21>
): ReturnType<typeof PublicKey.findProgramAddressSync> => {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("basic_info")],
    program.programId
  );
};

const Main: React.FC = () => {
  const wallet = useWallet();
  const { connection } = useConnection();
  const getProvider = () =>
    new AnchorProvider(connection, wallet, AnchorProvider.defaultOptions());

  const initializeVoting = async () => {
    const provider = getProvider();
    const program = new Program(idlObject, programId, provider) as Program<D21>;
    const [basicInfoPDA, basicInfoBump] = getBasicInfoPDA(program);
    await program.methods
      .initialize()
      .accounts({
        basicInfo: basicInfoPDA,
        initializer: wallet.publicKey,
      })
      .rpc({ commitment: "confirmed" });
    alert("Initialization succesfull!");
  };

  const getBasicInfo = async () => {
    const provider = getProvider();
    const program = new Program(idlObject, programId, provider) as Program<D21>;
    

  return (
    <div>
      <button
        className="group w-60 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
        onClick={() => initializeVoting()}
      >
        Initialize voting
      </button>
    </div>
  );
};

export default Main;
