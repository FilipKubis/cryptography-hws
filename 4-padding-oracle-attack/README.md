
## Description

The project contains a simple program which is designed to break a specific MAC then CBC encryption scheme via a padding oracle attack. The algorithm exposes the fact that different status codes are returned for wrong padding and for malformed message.
The code contains both the url of the service and the ciphertext it attempts to break. The program outputs the original message.


## Execution

Just perform the `cargo run` command to execute the program:
```console
    cargo run
```