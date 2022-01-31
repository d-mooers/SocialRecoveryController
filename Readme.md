# Social Recovery Rust Client

This handles the client-side work for a social recovery system.

## Social Recovery Algorithm

### Backup Process:

```
    1. Encrypt a mnemonic phrase using AES encryption, with a 52-byte key and 12-byte nonce both randomly generated.
    2. Combine the 52-byte key and 12-byte nonce to create a 64-byte secret that is [key][nonce].
    3. Use Shamir secret-sharing to create N shares of the 64-byte secret with a threshold k
    4. Query each guardian's public key, and use RSA encryption to encrypt the ith key with the public key of the ith guardian
    5. Post the encrypted mnemonic, threshold k, guardian public keys, and encrypted secret shares onchain.
```

### Recovery Process (Perspective of Recovering User):

```
    1. Create a new EOA, A
    2. Enter account into recovery mode
    3. Recieve text to confirm from Twilio
    4. Respond with public key of A
    5. If public key of A matches the account attemping to place account into recovery, continue.  Else, abort.
    6. Notify guardians to retreive their encrypted key shares, and prove you are the one requesting
    7. Wait for guardians to decrypt their keyshares
    8. Grab k keyshares from on chain that have been encrypted with the public key of the newly-made EOA
    9. Decrypt the k keyshares, and retrieve the original secret
    10. Split the 64-byte secret into a 52-byte key and 12-byte nonce
    11. Decrypt the mnemonic using the key and nonce
```

### Recovery Process (Perspective of Guardian):

```
    1. Recieve notification that action is required
    2. Confirm identity of requesting EOA
    3. encrypted_share <- Grab encrypted keyshare from chain
    4. decrypted_share <- Decrypt encrypted_share using private key
    5. new_share <- Encrypt decrypted_share using public key of requesting EOA
    6. Post new_share on chain
```

### Recovery Process (Case of Malicious Actor):

```
    1. Malicious actor B puts account A into recovery
    2. Smart contract emits RecoveryEntered event
    3. Hosted services detects event emission and parses account A's address from onchain
    4. Account A's phone number is queried from onchain data
    5. Account A receives notification text
    6. Account A notifies that they are not trying to recover account
    7. Account A taken out of recovery mode, recieves a 7-day locking period, and B is banned from ever putting A into recovery mode
```
