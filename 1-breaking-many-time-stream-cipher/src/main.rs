use hex::{decode};
use std::collections::HashMap;
use std::cmp;
use std::fs;

// define constant for ascii value of space
const SPACE: u8 = 32;

fn add_to_hashmap(byte: u8, hash_map: &mut HashMap<u8, u32>) {
    hash_map.entry(byte).or_insert(0);
    hash_map.insert(byte, 200 + hash_map[&byte]);
}

/**
 * The algorithm searches for information about the key using the following pattern:
 *      
 *      (b1 xor k) xor (b2 xor k) = b1 xor b2
 * 
 *      azAZ xor space >= 64
 * 
 *  This provides 2 possible candidates for that part of the key.
 */
fn find_key_candidates(ciphertext_bin_1: &Vec<u8>, ciphertext_bin_2: &Vec<u8>,
    ciphertexts_bin_xored: &Vec<u8>, key_candidates: &mut Vec<HashMap<u8, u32>>) {
    let length_smaller: usize = cmp::min(ciphertext_bin_1.len(), ciphertext_bin_2.len());
    for index in 0..length_smaller {

        let byte_xored = ciphertexts_bin_xored[index];
        if byte_xored >= 64 {

            let byte_1 = ciphertext_bin_1[index];
            let byte_2 = ciphertext_bin_2[index];

            let key_byte_candidate_1 = byte_1 ^ SPACE;
            let key_byte_candidate_2 = byte_2 ^ SPACE;

            let hash_map: &mut HashMap<u8, u32> = &mut key_candidates[index];
            add_to_hashmap(key_byte_candidate_1, hash_map);
            add_to_hashmap(key_byte_candidate_2, hash_map);
        }
    }
}

/**
 * Guesses the best candidate for a key
 * For each byte the best cadidate is chosen
 */
fn find_best_key(key_candidates: &mut Vec<HashMap<u8, u32>>) -> Vec<u8> {
    let mut best_key: Vec<u8> = Vec::new();

    for hash_map in key_candidates {

        let mut best_byte_key: u8 = 0;
        let mut largest_count: i32 = -1;

        for (key, value) in &*hash_map {
            if largest_count < *value as i32 {
                largest_count = *value as i32;
                best_byte_key = *key;
            }
        }
        best_key.push(best_byte_key);
    }
    return best_key;
}

/**
 * Bitwise xor of two binary strings (represented as byte vectors)
 */
fn xor_bytes(bin_1: &Vec<u8>, bin_2: &Vec<u8>) -> Vec<u8> {
    let bin_xored: Vec<u8> = bin_1.iter()
        .zip(bin_2)
        .map(|(x1, x2)| x1 ^ x2)
        .collect();
    
    return bin_xored;
}

/**
 * Decrypt the provided ciphertext using the provided key
 *  => print the plaintext
 */
fn decrypt_and_print(ciphertext_bin: &Vec<u8>, key: &Vec<u8>) {
    let plaintext_bin: Vec<u8> = xor_bytes(&ciphertext_bin, &key);
    let plaintext_chars: Vec<char> = plaintext_bin.iter().map(|x| *x as char).collect();
    let plaitext: String = plaintext_chars.iter().cloned().collect::<String>();
    println!("{}", plaitext);
}


fn main() {

    let ciphertexts_raw = fs::read_to_string("./input.txt")
        .expect("Please provide input.txt file with lines of hex strings in the project root");
    
    let ciphertexts: Vec<&str> = ciphertexts_raw.split("\n").collect();

    // save the cyphertexts into byte vectors
    let ciphertexts_bin: Vec<Vec<u8>> = ciphertexts.iter().map(|x| decode(x).unwrap()).collect();
    
    // intialize the key
    let longest_ciphertext = ciphertexts_bin.iter().map(|x| x.len()).max().unwrap();
    let mut key_candidates: Vec<HashMap<u8, u32>> = Vec::new();
    for _ in 0..longest_ciphertext {
        let key_hash_map: HashMap<u8, u32> = HashMap::new();
        key_candidates.push(key_hash_map);
    }
    
    // iterate over ciphertext pairs and look for weaknesses
    for (index_1, ciphertext_bin_1) in  ciphertexts_bin.iter().enumerate() {
        for (_, ciphertext_bin_2) in  ciphertexts_bin[index_1 + 1 ..].iter().enumerate() {
            
            let ciphertexts_bin_xored: Vec<u8> = xor_bytes(&ciphertext_bin_1, &ciphertext_bin_2);

            find_key_candidates(&ciphertext_bin_1, &ciphertext_bin_2,
                &ciphertexts_bin_xored, &mut key_candidates);
        }
    }

    // choose the best candidate for the key and print all the decrypted messages
    let key: Vec<u8> = find_best_key(&mut key_candidates);

    for (index, ciphertext_bin) in ciphertexts_bin.iter().enumerate() {

        // print the decrypted messages
        println!("\nThe \"decrpted\" plaintext number {} is:", index);
        decrypt_and_print(&ciphertext_bin, &key);
    }
}

