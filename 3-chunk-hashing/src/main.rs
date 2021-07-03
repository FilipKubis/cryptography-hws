use std::io::Read;
use std::fs::File;
use std::fs;

use sha2::{Sha256, Digest};
use hex;


/**
 * Reads contents of the file at path into a bytes vector
 */
fn read_binary(path: &str) -> Vec<u8> {
    let mut f = File::open(path).expect("Please provide the input file");
    let metadata = fs::metadata(path).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    buffer
}

/**
 * Returns hash of the provided chunk
 */
fn hash_chunk(chunk: &Vec<u8>) -> Vec<u8> {
     let mut hasher = Sha256::new();
     hasher.input(chunk);
     hasher.result().to_vec()
}

/**
 * Performs the sequential hashing algorithm
 * Outputs H0 (hash of the first chunk concatenated with H1)
 */
fn hash_sequence(sequence: &Vec<u8>) -> Vec<u8> {

    // initialize hash as empty
    let mut hash: Vec<u8> = vec![];

    // initialize the sequence length
    let mut sequence_lenght = sequence.len();

    while sequence_lenght > 0 {
        // compute chunk lenght
        let chunk_length = match sequence_lenght % 1024 {
            0 => 1024,
            n => n
        };

        // Take the last chunk and append the previous hash
        let mut chunk_with_hash: Vec<u8> = sequence[sequence_lenght-chunk_length..sequence_lenght].to_vec();
        chunk_with_hash.extend(&hash);

        // compute its hash
        hash = hash_chunk(&chunk_with_hash);

        // reduce sequence length by chunk length
        sequence_lenght -= chunk_length;
    }

    // return hash of the first block
    hash
}

fn main() {
    
    // Load input
    let input1 = read_binary("./input1.mp4");
    let input2 = read_binary("./input2.mp4");

    // Run the algorithm
    let hash1 = hash_sequence(&input1);
    let hash2 = hash_sequence(&input2);

    // Print the hashes in hex
    println!("Input 1 has H0 of : {}", hex::encode(hash1));
    println!("Input 2 has H0 of : {}", hex::encode(hash2));

}
