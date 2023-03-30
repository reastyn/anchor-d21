# Implementation of D21 method in Solana

The exact same implementation of D21 that was done in Ethereum at class NI-BLO is being done in Solana. The only difference is that the D21 contract is deployed on the Solana blockchain.

Unfortunately, that lead to some un-solana like behaviour, because you have to create a program per election.

Otherwise, the implementation works correctly.

The program has working both trdelnik and anchor tests and is deployed on the devnet. The address address is specified in the frontend and `Anchor.toml` file.

I did not have time to really polish the UI so pardon the error messages and the lack of a great user experience.

## D21 implementation requirements
"Janečkova metoda D21" is a modern voting system, which allows more accurate voting. You can learn more about it here: https://www.ih21.org/o-metode. In our exercise, we want to achieve the following use cases:

- UC1 - Everyone can register a subject (e.g. political party)
- UC2 - Everyone can list registered subjects
- UC3 - Everyone can see the subject’s results
- UC4 - Only the owner can add eligible voters
- UC5 - Every voter has 2 positive and 1 negative vote
- UC6 - Voter can not give more than 1 vote to the same subject
- UC7 - Negative vote can be used only after 2 positive votes
- UC8 - Voting ends after 7 days from the contract deployment
