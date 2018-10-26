/*
  This code finds a solution to the NPR puzzle presented on Oct 21, 2018.

  A (paraphrased) statement of the problem:
    Given the letters in the word 'beermouth', contruct a three by three matrix of words where all
    the verticals, horizontals, and diagonals form three letter words.

  This solution is a (nearly line for line) translation of the solution written y Pat Rondon here:
  https://gist.github.com/pat-rondon/9568e2840d9af31030bf4c9d4b7ebae8
*/

use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;

extern crate permutohedron;
use permutohedron::Heap;

extern crate itertools;
use itertools::any;
use itertools::Itertools;

fn is_word<'a, I>(letters: I, words: &HashSet<String>) -> bool
where
    I: Iterator<Item = &'a char>,
{
    let word: String = letters.collect();
    words.contains(&word)
}

fn main() {
    let f = match File::open("dictionary.txt") {
        Ok(f) => f,
        Err(why) => panic!("Failed to open links.txt: {}", Error::description(&why)),
    };

    let words: HashSet<String> = BufReader::new(f)
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| s.len() == 3)
        .collect();

    let mut letters: Vec<char> = "beermouth".chars().collect();
    let heap = Heap::new(&mut letters);

    for perm in heap {
        let rows = perm.iter().chunks(3);
        if any(&rows, |letters| !is_word(letters, &words)) {
            continue;
        }
        let mut missing = false;
        let mut iter_perm = perm.iter();
        for _ in 0..3 {
            let word: String = iter_perm.clone().step_by(3).collect();
            if !words.contains(&word) {
                missing = true;
                break;
            }
            iter_perm.next();
        }
        if missing {
            continue;
        }

        let diagonal_top_left: String = [perm[0], perm[4], perm[8]].iter().collect();
        if !words.contains(&diagonal_top_left) {
            continue;
        }

        let diagonal_top_right: String = [perm[2], perm[4], perm[6]].iter().collect();
        if !words.contains(&diagonal_top_right) {
            continue;
        }

        println!("Found solution:");
        for ii in 0..3 {
            println!(
                " {} {} {}",
                perm[3 * ii],
                perm[3 * ii + 1],
                perm[3 * ii + 2]
            );
        }
    }
}
