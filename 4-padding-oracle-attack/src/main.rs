use futures::future::join_all;

/**
 * Bitwise xor of two binary strings (represented as byte vectors)
 */
fn xor_bytes(bin_1: &Vec<u8>, bin_2: &Vec<u8>) -> Vec<u8> {
    bin_1.iter()
        .zip(bin_2)
        .map(|(x1, x2)| x1 ^ x2)
        .collect()
}

/**
 * Get the http response status for the provided ciphertext
 */
async fn get_status(
    ciphertext_bytes: Vec<u8>,
    client: &reqwest::Client
) -> Result<reqwest::StatusCode, Box<dyn std::error::Error>> {
    
    // URL of website which leaks information via padding oracle attack
    let url = "http://crypto-class.appspot.com/po?er=".to_owned();

    let ciphertext = hex::encode(ciphertext_bytes);
    Ok(client.get(url.clone() + &ciphertext).send().await?.status())
}

/**
 * (Simplifies the subsequent xor operation)
 * Expands the padding for the current step
 */
fn expand_padding(ciphertext_size: usize, padding_size: usize) -> Vec<u8> {
    let mut padding: Vec<u8> = vec![0; ciphertext_size];

    for i in 0..padding_size {
        padding[ciphertext_size - 17 - i as usize] = padding_size as u8;
    }

    return padding
}

/**
 * (Simplifies the subsequent xor operation)
 * Expands the already correctly guessed values
 */
fn expand_guesses(ciphertext_size: usize, correct_guesses: &Vec<u8>) -> Vec<u8> {
    let mut guesses_expanded: Vec<u8> = vec![0; ciphertext_size];

    for i in 0..16 {
        guesses_expanded[ciphertext_size - 17 - i as usize] = correct_guesses[15 - i];
    }

    return guesses_expanded;
}

/**
 * Guesses the byte specified by the index in the last block of the passed ciphertext
 */
async fn guess_byte(
    ciphertext_bytes: &[u8],
    correct_guesses: &Vec<u8>,
    index: usize,
    client: &reqwest::Client
) -> Result<u8, Box<dyn std::error::Error>> {

    let mut chosen_ciphertext: Vec<u8> = ciphertext_bytes.to_vec();
    let len_ciphertext = chosen_ciphertext.len();

    // get expanded padding and expanded correct guesses
    let padding_size = index + 1;
    let expanded_padding = expand_padding(len_ciphertext, padding_size);
    let expanded_correct_guesses = expand_guesses(len_ciphertext, &correct_guesses);

    chosen_ciphertext = xor_bytes(&chosen_ciphertext, &expanded_padding);
    chosen_ciphertext = xor_bytes(&chosen_ciphertext, &expanded_correct_guesses);

    // Call the server asynchronously for each possible character value
    let index_in_ciphertext = len_ciphertext - 16 - padding_size;
    let mut promises = Vec::new();
    for guess in 0..=255 {
        // xor the guess with the ciphertext
        chosen_ciphertext[index_in_ciphertext] = chosen_ciphertext[index_in_ciphertext] ^ guess;

        // call the request
        promises.push(get_status(chosen_ciphertext.clone(), client));

        // revert the ciphertext
        chosen_ciphertext[index_in_ciphertext] = chosen_ciphertext[index_in_ciphertext] ^ guess;
    }

    let status_codes: Vec<reqwest::StatusCode> = join_all(promises)
        .await
        .into_iter()
        .map(|x| x.unwrap())
        .collect();

    // await the promises
    for guess in 0..=255 {
        let status = status_codes[guess];

        // if any status is 404 or 200 that means a correct guess
        if status == 404 {
            return Ok(guess as u8);
        }    
    }
    return Ok(0);
}

/**
 * Guesses the encryption of a ciphertext block;
 * Decrypts the padding first, then the message ending
 */
async fn poa_last_block(
    ciphertext_bytes: &[u8],
    client: &reqwest::Client
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {

    let mut correct_guesses: Vec<u8> = vec![0; 16];

    let padding = guess_byte(ciphertext_bytes, &correct_guesses, 0, client).await?;

    for i in 0..padding {
        let index = 15 - i;
        correct_guesses[index as usize] = padding;
    }

    // guess each byte
    for i in padding..16 {
        let index = 15 - i;
        correct_guesses[index as usize] = guess_byte(ciphertext_bytes, &correct_guesses, i as usize, client).await?;
    } 

    return Ok(correct_guesses);
}

/**
 * Guesses the encryption of a ciphertext block
 */
async fn poa_block(
    ciphertext_bytes: &[u8],
    client: &reqwest::Client
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {

    let mut correct_guesses: Vec<u8> = vec![0; 16];

    // guess each byte
    for i in 0..16 {
        let index = 15 - i;
        correct_guesses[index] = guess_byte(ciphertext_bytes, &correct_guesses, i, client).await?;
    }

    return Ok(correct_guesses);
}

/**
 * Returns String based on provided binary string representing ascii text
 */
fn bytes_to_ascii(bytes: Vec<u8>) -> String {
    let plaintext_chars: Vec<char> = bytes.iter().map(|x| *x as char).collect();
    let plaintext: String = plaintext_chars.iter().cloned().collect::<String>();
    return plaintext;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // The MAC then CBC ciphertext, which we try to decrypt
    let ciphertext = "f20bdba6ff29eed7b046d1df9fb7000058b1ffb4210a580f748b4ac714c001bd4a61044426fb515dad3f21f18aa577c0bdf302936266926ff37dbf7035d5eeb4";
    let ciphertext_bytes = hex::decode(ciphertext)?;

    // create a http request client
    let client = reqwest::Client::new();

    // initialize empty vector for the final message
    let n_blocks = (ciphertext_bytes.len() / 16) - 1;
    let mut message_bytes: Vec<u8> = Vec::new();

    // Run the poa for each block
    for i in 0..(n_blocks - 1) {
        let block_end = (i + 2) * 16;
        let message_block = poa_block(&ciphertext_bytes[0..block_end], &client).await?;
        message_bytes.extend(message_block.clone());
        println!("The message block no. {} is: {}", i, bytes_to_ascii(message_block));
    }

    // Run the poa for the last block (uses different logic due to padding)
    let i = n_blocks - 1;
    let block_end = (i + 2) * 16;
    let message_block = poa_last_block(&ciphertext_bytes[0..block_end], &client).await?;
    message_bytes.extend(message_block.clone());
    println!("The message block no. {} is: {}", i, bytes_to_ascii(message_block));

    println!("The message is: \"{}\"", bytes_to_ascii(message_bytes));
    Ok(())
}
