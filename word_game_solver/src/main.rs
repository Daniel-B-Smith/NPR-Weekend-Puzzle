use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::vec::Vec;

extern crate solver_lib;
use solver_lib::*;

fn main() {
    let f = match File::open("dictionary.txt") {
        Ok(f) => f,
        Err(why) => panic!("Failed to open links.txt: {}", Error::description(&why)),
    };

    let words: Vec<String> = BufReader::new(f)
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| s.len() == 5)
        .filter(|s| is_unique(s))
        .collect();

    let mut combos = letter_combos(&words);
    println!("Ready?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    while combos.len() > 1 {
        input.clear();
        let guess = gen_guess(&combos);
        println!("My next guess is: {}", guess);
        io::stdin().read_line(&mut input).unwrap();
        let num_matching: i32 = input.trim().parse().unwrap();
        if num_matching < 0 || num_matching > 5 {
            println!("No cheating!");
            continue;
        }
        filter_matches(guess, num_matching, &mut combos);
        println!("{} letter combos remaining.", combos.len());
    }

    if combos.len() == 1 {
        println!("My guesses:");
        for combo in combos {
            for word in combo.1 {
                println!("{}", word);
            }
        }
    } else {
        println!("You failed!");
    }
}
