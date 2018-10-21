use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;

// Convert the word to a map from charaters to the number of times that character appears.
fn construct_letter_map(word: &str) -> HashMap<char, u8> {
    let mut map = HashMap::new();
    for letter in word.chars() {
        *map.entry(letter).or_insert(0) += 1
    }
    map
}

// Compare if `left` is a subset of `right`.
fn letter_map_less_than(left: &HashMap<char, u8>, right: &HashMap<char, u8>) -> bool {
    for (letter, l_count) in left {
        match right.get(letter) {
            Some(r_count) => if l_count > r_count {
                return false;
            },
            _ => return false,
        }
    }
    true
}

fn main() {
    let f = match File::open("dictionary.txt") {
        Ok(f) => f,
        Err(why) => panic!("Failed to open links.txt: {}", Error::description(&why)),
    };

    // Construct a vector of three letter words that are a subset of beermouth.
    let source_word = construct_letter_map("beermouth");
    let words: Vec<String> = BufReader::new(f)
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| s.len() == 3)
        .filter(|s| letter_map_less_than(&construct_letter_map(s), &source_word))
        .map(|s| s)
        .collect();

    for word in words {
        println!("{}", word);
    }
}
