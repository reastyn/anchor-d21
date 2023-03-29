# Implementation of D21 method in Solana

The exact same implementation of D21 that was done in Ethereum at class NI-BLO is being done in Solana. The only difference is that the D21 contract is deployed on the Solana blockchain.

Unfortunately, that lead to some un-solana like behaviour, because you have to create a program per election.

Otherwise, the implementation works correctly.

The program has working both trdelnik and anchor tests and is deployed on the devnet. The address address is specified in the frontend and `Anchor.toml` file.

I did not have time to really polish the UI so pardon the error messages and the lack of a great user experience.
