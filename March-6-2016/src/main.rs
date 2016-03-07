use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;

fn collect_matching_words(mut words: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut collected_words = HashMap::new();
    for word in words.drain(..) {
        let (prefix, suffix) = word.split_at(1);
        collected_words.entry(suffix.to_owned()).or_insert(Vec::new()).push(prefix.to_owned())
    }
    collected_words
}

fn find_row(letter: &str) -> i8 {
    static first: [&'static str; 10] = ["q", "w", "e", "r", "t", "y", "u", "i", "o", "p"];
    static second: [&'static str; 9] = ["a", "s", "d", "f", "g", "h", "j", "k", "l"];
    static third: [&'static str; 7] = ["z", "x", "c", "v", "b", "n", "m"];
    if first.contains(&letter) {
        return 1;
    } else if second.contains(&letter) {
        return 2;
    } else if third.contains(&letter) {
        return 3;
    }
    unreachable!()
}

fn main() {
    let f = match File::open("some_words.txt") {
        Ok(f) => f,
        Err(why) => panic!("Failed to open links.txt: {}", Error::description(&why)),
    };
    let words: Vec<String> = BufReader::new(f)
                                 .lines()
                                 .map(|s| s.unwrap())
                                 .filter(|s| s.len() == 4)
                                 .map(|s| s)
                                 .collect();
    let suffix_to_prefix: HashMap<String, Vec<String>> = collect_matching_words(words)
                                                             .into_iter()
                                                             .filter(|&(_, ref v)| v.len() > 4)
                                                             .collect();
    for (suffix, prefixes) in suffix_to_prefix {
        let mut rows_map: HashMap<i8, i8> = HashMap::new();
        for prefix in prefixes.iter() {
            *rows_map.entry(find_row(&prefix)).or_insert(0) += 1;
        }
        for (row, count) in rows_map {
            if count >= 5 {
                println!("Found:");
                for prefix in prefixes.iter() {
                    if find_row(&prefix) == row {
                        println!("{}{}", prefix, suffix);
                    }
                }
            }
        }
    }
}
