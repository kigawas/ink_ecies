# INK ECIES

ECIES implementation in ink! contract with chain extension.

## ECIES in a nutshell

ECIES (Elliptic Curve Integrated Encryption Scheme) is a hybrid encryption protocol that ensures secure data exchange. It combines symmetric encryption for efficiency and asymmetric encryption for enhanced security.

ECIES offers strong protection against eavesdropping and unauthorized access, making it a competitive choice for secure communication over networks and in cryptographic applications.


## Technical details

### ECIES

Secp256k1 and XChaCha20-Poly1305 are used for asymmetric shared secret exchange and symmetric encryption/decryption. Compared to the popular AES-256-GCM, XChaCha20 is fast and constant-time without hardware acceleration (resistant to cache-timing attacks), and it also has longer nonce length.

Generally there are two steps:

1. The sender generates an ephemeral key pair, derives a shared secret with the recipient's public key and the ephemeral secret key. The sender encrypts the data with the shared secret under XChaCha20-Poly1305.
2. The recipient derives the shared secret with the recipient's secret key and the ephemeral public key.

The payload is as below:

```
+-------------------------------+----------+----------+-----------------+
| 65 Bytes                      | 24 Bytes | 16 Bytes | == data size    |
+-------------------------------+----------+----------+-----------------+
| Sender Public Key (ephemeral) | Nonce    | Tag      | Encrypted data  |
+-------------------------------+----------+----------+-----------------+
|           Secp256k1           |          XChaCha20-Poly1305           |
+-------------------------------+---------------------------------------+
```

### Chain extension

Substrate chain extension allows developers to call function provided by the chain in smart contracts. This feature enables us to implement ECIES in Astar, and call it in ink! contract. Solidity won't support this feature.

Note that securely random bytes generation is a pretty challenging problem on blockchains, we have to use the insecure randomness pallet provided by substrate.

We use the message as the seed to generate a nonce, then use the nonce as the seed to generate the ephemeral secret key to make it "more random".

```rust
    let nonce_output = T::Randomness::random(&msg).0; // TODO: VRF
    let nonce = nonce_output.encode();

    let sk_output = T::Randomness::random(&nonce).0; // TODO: VRF
    let sk = sk_output.encode();
```
