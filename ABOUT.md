What it does

ECIES implementation in ink! contract with chain extension. Users can use it to encrypt messages without gas fee, or they can save the result on the contract. They can decrypt the message with their private key later somewhere else. The maximum size of the message is around 6K bytes.


The problem it solves

There are not many cryptographic applications on Astar, because many cryptographic algorithms are time-consuming and cannot be implemented in ink! contract. It showed the potential of cryptographic applications on Astar.


Challenges I ran into

Random bytes generation. I used a trick to make the generated private cannot be computed directly, but it still needs verifiable random functions (VRF) on Astar or Substrate.


Technologies I used

- Substrate chain extension
- ink! contract
- Cryptography


How we built it

Due to the limit of Substrate/Astar (contract size limit, gas limit, etc.), we cannot directly implement ECIES in ink! contract. Instead, we implement it with a chain extension (https://github.com/kigawas/Astar/compare/master...ecies-ext) in Astar, then implement a contract to call the function provided by the chain extension.


What we learned

At first I tried to implement it in ink! contract, but it's impossible to implement it in ink! contract due to the limit. So I resorted to chain extension, and it just works.


What's next for

- Integrate with ed25519, which is natively supported by substrate
- Improve the smart contract, for example, to add ownership management, charge fee and so on
