
## Description

The project contains a simple program which computes a discrete logarithm of 2 large numbers (modulo large prime).
The program expects number such that value of the discrete log is less than 2^40.
The file input.txt contains the input to the program in the form of three numbers p, g, h respectively, each on its own line. The program computes log_g h (mod p).


## Execution

Just perform the `cargo run` command to execute the program:
```console
    cargo run
```