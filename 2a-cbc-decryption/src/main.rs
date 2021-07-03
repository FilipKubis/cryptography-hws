use std::fs;
use aes::Aes128;
use aes::cipher::{
    BlockDecrypt, NewBlockCipher,
    generic_array::GenericArray
};
use hex;


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
 * Returns String based on provided binary string representing ascii text
 */
fn bytes_to_ascii(bytes: Vec<u8>) -> String {
    let plaintext_chars: Vec<char> = bytes.iter().map(|x| *x as char).collect();
    let plaintext: String = plaintext_chars.iter().cloned().collect::<String>();
    return plaintext;
}

/**
 * Decrypts CBC cyphertext
 */
fn decrypt(key_hex: String, ciphertext_hex: String) -> String {

    // convert all to byte arrays
    let key: Vec<u8> = hex::decode(key_hex).unwrap();
    let ciphertext: Vec<u8> = hex::decode(ciphertext_hex).unwrap();

    // initialize cipher
    let cipher = Aes128::new(&GenericArray::from_slice(&key));

    // run the decryption algorithm
    let n_blocks: i32 = ((ciphertext.len() as i32) / 16) - 1;

    // convert message to ascii and return it
    let mut message: Vec<u8> = Vec::new();
    for i in 0..n_blocks {

        // get nonce
        let nonce_start = 16 * (i) as usize;
        let nonce_end =  nonce_start + 16 as usize;
        let nonce = ciphertext[nonce_start..nonce_end].to_vec();

        // get ciphertext block
        let block_start = 16 * (i + 1) as usize;
        let block_end =  block_start + 16 as usize;
        let mut ciphertext_block = GenericArray::clone_from_slice(&ciphertext[block_start..block_end]);

        // decrypt block and add it to message
        cipher.decrypt_block(& mut ciphertext_block);
        let decrypted_block = xor_bytes(&ciphertext_block.to_vec() , &nonce);
        message.extend(decrypted_block);
    }

    // return plaintext decrypted message
    return bytes_to_ascii(message);
}

/**
 * Reads a file and returns vector of lines from the file
 */
fn read_lines(path: &str) -> Vec<String> {
    let content_raw = fs::read_to_string(path)
        .expect(format!("Please provide {} file with lines of hex strings in the project root", path).as_str());

    return content_raw.split('\n').map(|x| x.to_string()).collect();
}

fn main() {

    // load ciphertexts and keys
    let ciphertexts: Vec<String> = read_lines("./input_ciphertexts.txt");
    let keys: Vec<String> = read_lines("./input_keys.txt");

    // for each pair, dedcrypt the message and print the acsii plaintext
    for (index, (ciphertext, key)) in ciphertexts.into_iter().zip(keys).enumerate() {
        
        let message: String = decrypt(key, ciphertext);

        println!("\nEncrypted message number {}:", index);
        println!("{}", message);
    } 
}
