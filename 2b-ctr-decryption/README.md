
## Description

The project contains a program which decrypts counter block cipher (CTR) ciphertext, given a key.
The CTR uses 16 byte nonce included at the start of each ciphertext.
The encryption algorithm used is AES.

The inputs are in input-*.txt files. Both contain n lines of either keys or ciphertexts.

## Execution

Just perform the `cargo run` command to execute the program:
```console
    cargo run
```