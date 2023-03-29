import type { NextPage } from "next";
import Head from "next/head";
import { D21View } from "../views";

const D21: NextPage = (props) => {
  return (
    <div>
      <Head>
        <title>Solana Scaffold</title>
        <meta
          name="description"
          content="Basic Functionality"
        />
      </Head>
      <D21View />
    </div>
  );
};

export default D21;
