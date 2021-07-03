use std::fs;
use std::cmp;
use aes::Aes128;
use aes::cipher::{
    BlockEncrypt, NewBlockCipher,
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
 * Increments a binary string (represented by a bytes vector)
 */
fn increment_nonce(nonce_orig: &Vec<u8>) -> Vec<u8> {

    let mut nonce_new = nonce_orig.clone();

    // check if the increment overflows to next bytes, break otherwise
    for i in (0..16).rev() {
        if nonce_new[i] == 255 {
            nonce_new[i] = 0;
        }
        else{
            nonce_new[i] += 1;
            break;
        }
    }
    return nonce_new;
}

/**
 * Decrypts CTR ciphertext
 */
fn decrypt(key_hex: String, ciphertext_hex: String) -> String {

    // convert all to byte arrays
    let key: Vec<u8> = hex::decode(key_hex).unwrap();
    let ciphertext: Vec<u8> = hex::decode(ciphertext_hex).unwrap();

    // get nonce (iv)
    let mut nonce = ciphertext[0..16].to_vec();

    // initialize cipher
    let cipher = Aes128::new(&GenericArray::from_slice(&key));

    // run the decryption algorithm
    let n_blocks: i32 = ((ciphertext.len() as f64) / 16.0).ceil() as i32 - 1;

    // convert message to ascii and return it
    let mut message: Vec<u8> = Vec::new();
    for i in 0..n_blocks {

        // get ciphertext block
        let block_start = 16 * (i + 1) as usize;
        let block_end =  cmp::min(block_start + 16, ciphertext.len());
        let ciphertext_block = ciphertext[block_start..block_end].to_vec();

        // decrypt block and add it to message
        let mut nonce_mutable = GenericArray::clone_from_slice(&nonce);
        cipher.encrypt_block(& mut nonce_mutable);
        let decrypted_block = xor_bytes(&nonce_mutable.to_vec(), &ciphertext_block);
        message.extend(decrypted_block);

        // increment nonce
        nonce = increment_nonce(&nonce);
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
