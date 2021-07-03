use rug::{Assign, Integer};
use std::fs;
use std::collections::HashMap;

/**
 * Reads a file and returns vector of lines from the file
 */
fn read_lines(path: &str) -> Vec<String> {
    let content_raw = fs::read_to_string(path)
        .expect(format!("Please provide {} file with lines of hex strings in the project root", path).as_str());

    return content_raw.split('\n').map(|x| x.to_string()).collect();
}

/**
 * Self-explanatory :)
 */
fn divide_h_by_g_to_x(p: &Integer, g: &Integer, h: &Integer, x: i64) -> Integer {

    let g_to_x = Integer::from(g.pow_mod_ref(&Integer::from(x), &p).unwrap());
    let g_to_x_inv = Integer::from(g_to_x.invert(&p).unwrap());

    Integer::from( Integer::from(h * g_to_x_inv) % p)
}

/**
 * Creates a hash map of all possible values of the left-hand side
 * with the exponents as the hashmap values
 */
fn left_hand_side(p: &Integer, g: &Integer, h: &Integer) -> HashMap<Integer, i64> {

    // initialize the empty hashmap
    let mut hash_map = HashMap::new();

    // for each possible exponent, compute the value of the lhs
    for x in 0..=2i64.pow(20) {
        let value: Integer = divide_h_by_g_to_x(p, g, h, x);
        hash_map.insert(value, x);
    }

    return hash_map;
}



/**
 * Tries to find match in the hash table
 * for each possible value of the right-hand side
 */
fn meet_in_the_middle(hash_map: HashMap<Integer, i64>, p: &Integer, g: &Integer) -> i64 {

    let b = 2i64.pow(20);

    // look for a match in the hashmap
    for x in 0..=b {
        let g_to_base = Integer::from(g.pow_mod_ref(&Integer::from(b), &p).unwrap());
        let value: Integer = Integer::from(g_to_base.pow_mod_ref(&Integer::from(x), &p).unwrap());
        
        if hash_map.contains_key(&value) {
            let x1 = hash_map.get(&value).unwrap();
            return x * b + x1;
        }
    }
    return -1;
}


fn main() {

    // load the input - prime p and two large integers h and g
    let numbers_raw: Vec<String> = read_lines("./input.txt");
    let mut p: Integer = Integer::new();
    p.assign(Integer::parse(numbers_raw[0].clone()).unwrap());
    let mut g: Integer = Integer::new();
    g.assign(Integer::parse(numbers_raw[1].clone()).unwrap());
    let mut h: Integer = Integer::new();
    h.assign(Integer::parse(numbers_raw[2].clone()).unwrap());

    let hash_map = left_hand_side(&p, &g, &h);
    let discrete_log = meet_in_the_middle(hash_map, &p, &g);

    println!("The log (base g) of h (mod p) is: {}", discrete_log);
}
