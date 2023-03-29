import { useWallet } from "@solana/wallet-adapter-react";
import React, { PropsWithChildren } from "react";

const WalletProtector: React.FC<PropsWithChildren> = ({ children }) => {
  const { publicKey } = useWallet();

  if (!publicKey) {
    return <div>Connect your wallet first before interacting</div>;
  }

  return <>{children}</>;
};

export default WalletProtector;
