use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = match File::open("dictionary.txt") {
        Ok(f) => f,
        Err(why) => panic!("Failed to open dictionary.txt: {}", why),
    };

    let mut fours = HashSet::new();
    let mut threes = HashSet::new();
    let mut eights = Vec::new();
    for word in BufReader::new(f).lines().map(|s| s.unwrap()) {
        // Use bytes so that we can use slice indexing.
        let word = word.into_bytes();
        match word.len() {
            3 => {
                threes.insert(word);
            }
            4 => {
                fours.insert(word);
            }
            8 => eights.push(word),
            _ => (),
        };
    }

    for word in eights {
        if !threes.contains(&word[..3]) {
            continue;
        }
        if !fours.contains(&word[4..]) {
            continue;
        }

        println!(
            "{} {} {}",
            std::str::from_utf8(&word).unwrap(),
            std::str::from_utf8(&word[..3]).unwrap(),
            std::str::from_utf8(&word[4..]).unwrap()
        );
    }
}
