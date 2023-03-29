import { AnchorProvider, Program } from "@project-serum/anchor";
import { useConnection, useWallet } from "@solana/wallet-adapter-react";
// import idl from "./../../../target/idl/d21.json"; // Just so you don't have to build it :)
import idl from "./d21.json";
import { D21 } from "./d21";
import { PublicKey } from "@solana/web3.js";
import { useEffect, useState } from "react";

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
  const [selectedProgramId, setSelectedProgramId] = useState(programId);
  const [basicInfo, setBasicInfo] =
    useState<
      Awaited<ReturnType<Program<D21>["account"]["basicInfo"]["fetch"]>>
    >(null);
  const [subjects, setSubjects] = useState(null);
  const [subjectName, setSubjectName] = useState("");
  const getProvider = () =>
    new AnchorProvider(connection, wallet, AnchorProvider.defaultOptions());

  const getBasicInfo = async (selectedProgramId: string = programId) => {
    setSelectedProgramId(selectedProgramId);
    const provider = getProvider();
    const program = new Program(
      idlObject,
      selectedProgramId,
      provider
    ) as Program<D21>;
    const [basicInfoPDA] = getBasicInfoPDA(program);
    try {
      const basicInfo = await program.account.basicInfo.fetch(basicInfoPDA);
      setBasicInfo(basicInfo);
    } catch (e) {}
  };

  const initializeVoting = async () => {
    const provider = getProvider();
    const program = new Program(idlObject, programId, provider) as Program<D21>;
    const [basicInfoPDA, basicInfoBump] = getBasicInfoPDA(program);
    try {
      await program.methods
        .initialize()
        .accounts({
          basicInfo: basicInfoPDA,
          initializer: wallet.publicKey,
        })
        .rpc({ commitment: "confirmed" });
      alert("Initialization succesfull!");
    } catch (e) {
      if (
        e
          .toString()
          .includes("Error processing Instruction 0: custom program error: 0x0")
      ) {
        alert(
          "You initialized election already, loading your current election"
        );
      } else {
        alert(e);
      }
    }
    await getBasicInfo();
  };

  const getSubjects = async () => {
    const provider = getProvider();
    const program = new Program(idlObject, programId, provider) as Program<D21>;

    const [basicInfoPDA] = getBasicInfoPDA(program);
    const fetchedSubjects = await program.account.subjectAccount.all();
    setSubjects(fetchedSubjects);
    console.log(fetchedSubjects);
  };

  useEffect(() => {
    if (subjects === null) {
      getSubjects();
    }
  }, []);

  const onAddSubject = async () => {
    if (subjectName === "") {
      alert("Subject name cannot be empty!");
      return;
    }
    const provider = getProvider();
    const program = new Program(idlObject, programId, provider) as Program<D21>;

    const [subjectPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("subject"), wallet.publicKey.toBuffer()],
      program.programId
    );
    const [basicInfoPDA] = getBasicInfoPDA(program);
    try {
      await program.methods
        .addSubject(subjectName)
        .accounts({
          basicInfo: basicInfoPDA,
          initializer: wallet.publicKey,
          subject: subjectPDA,
        })
        .rpc({ commitment: "confirmed" });
    } catch (e) {
      if (
        e
          .toString()
          .includes("Error processing Instruction 0: custom program error: 0x0")
      ) {
        alert("You added yourself as subject already");
      } else {
        alert(e);
      }
    }

    await getSubjects();
  };

  const selectElection = () => {
    const selectedProgramId = prompt(
      "Enter the program id of the election you want to open"
    );
    if (!selectedProgramId) return;
    getBasicInfo(selectedProgramId);
  };

  const onNameChange = (event) => {
    setSubjectName(event.target.value);
  };

  const addVoter = async () => {
    const voterPublicKey = prompt("Enter the public key of the voter");
    if (!voterPublicKey) return;
    const provider = getProvider();
    const program = new Program(idlObject, programId, provider) as Program<D21>;

    const [basicInfoPDA] = getBasicInfoPDA(program);
    let voter = null;
    try {
      voter = new PublicKey(voterPublicKey);
    } catch (e) {
      alert("Invalid public key");
    }
    if (!voter) return;
    const [voterPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("voter"), voter.toBuffer()],
      program.programId
    );
    try {
      await program.methods
        .addVoter(voter)
        .accounts({
          basicInfo: basicInfoPDA,
          initializer: wallet.publicKey,
          voter: voterPDA,
        })
        .rpc({ commitment: "confirmed" });
    } catch (e) {
      alert(e);
    }
    alert("Voter added!");
  };

  const vote = async (
    selectedProgramId: string,
    subject: PublicKey,
    isPositiveVote: boolean
  ) => {
    const provider = getProvider();
    const program = new Program(
      idlObject,
      selectedProgramId,
      provider
    ) as Program<D21>;

    const [basicInfoPDA] = getBasicInfoPDA(program);
    const [voterPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("voter"), wallet.publicKey.toBuffer()],
      program.programId
    );
    const [subjectPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("subject"), subject.toBuffer()],
      program.programId
    )
    try {
      await program.methods
        .vote(subject, isPositiveVote)
        .accounts({
          basicInfo: basicInfoPDA,
          initializer: wallet.publicKey,
          voter: voterPDA,
          subject: subjectPDA,
        })
        .rpc({ commitment: "confirmed" });
    } catch (e) {
      if (e.toString().includes("AccountNotInitialized. Error Number: 3012")) {
        alert("Owner of this election did not add you as voter");
        alert(e);
      } else {
        alert(e);
      }
    }
    await getBasicInfo();
    await getSubjects();
  };

  if (basicInfo === null) {
    return (
      <div>
        <button
          className="group w-60 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
          onClick={() => initializeVoting()}
        >
          Create this program&apos;s program
        </button>
        <button
          className="group w-60 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
          onClick={() => getBasicInfo()}
        >
          Open program&apos;s election
        </button>
        <button
          className="group w-60 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
          onClick={() => selectElection()}
        >
          Open other election
        </button>
      </div>
    );
  }

  return (
    <div>
      <div>
        <p>Owner of voting: {basicInfo.owner.toBase58()}</p>
        <p>
          The voting ends at:{" "}
          {new Date(basicInfo.endDate.toNumber() * 1000).toString()}
        </p>
        {basicInfo.owner.equals(wallet.publicKey) && (
          <button
            className="group w-60 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
            onClick={() => addVoter()}
          >
            Add voter to your election
          </button>
        )}
      </div>
      <hr className="h-px my-14 bg-[#d946ef] border-0" />
      <div>
        <span className="text-xl font-bold text-[#d946ef]">
          Add yourself as subject to vote for:
        </span>
        <form>
          <label htmlFor="fname">Your name:</label>
          <input
            type="text"
            id="fname"
            name="fname"
            className="bg-[#0a0a29] border-[#d946ef] border-2 p-2"
            onChange={onNameChange}
          />
          <input
            className="px-8 m-2 btn animate-pulse bg-gradient-to-br from-indigo-500 to-fuchsia-500 hover:from-white hover:to-purple-300 text-black"
            type="submit"
            value="Submit"
            onClick={(e) => {
              e.preventDefault();
              onAddSubject();
            }}
          />
        </form>
      </div>
      <div className="mt-4">
        Subjects:
        {subjects?.map((subject) => {
          return (
            <div
              key={subject.publicKey.toBase58().toString()}
              className="text-left mt-2 border-2 color-white p-4"
            >
              <p>
                Subject public key: {subject.publicKey.toBase58().toString()}
              </p>
              <p>Subject name: {subject.account.name}</p>
              <p>Subject votes: {subject.account.votes.toString()}</p>
              <button
                className="group m-2 btn bg-green-500 hover:from-white hover:to-purple-300 text-black"
                onClick={() => vote(selectedProgramId, subject.account.pubkey, true)}
              >
                + Vote Positively
              </button>
              <button
                className="group m-2 btn bg-red-500 text-white hover:from-white hover:to-purple-300 text-black"
                onClick={() =>
                  vote(selectedProgramId, subject.account.pubkey, false)
                }
              >
                - Vote Negatively
              </button>
            </div>
          );
        })}
      </div>
      <hr className="h-px my-14 bg-[#d946ef] border-0" />
    </div>
  );
};

export default Main;
