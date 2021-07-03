
## Description

The projects contains a simple program which enables to continuosly check integrity of chunks of files (as they are downloaded), which enables for example to play a video as it downloads.
The program takes a video as an input file.
The input file is split into N 1024 byte chunks.

The hash of the Nth chunk is concatenated to the N-1st chunk.
The hash of N-1st chunk (which now contains the hash of the last chunk) is concatenated to the N-2nd chunk.
This algorithm is repeated until chunk 1 and the program outputs the hash (in hex) of the first chunk (just to check the correctness of the algorithm).
The hash function used is SHA256.

The inputs are input1.mp4 and input2.mp4 files (can be of any format really). 

## Execution

Just perform the `cargo run` command to execute the program:
```console
    cargo run
```