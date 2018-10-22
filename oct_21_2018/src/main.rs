/*
  This code finds a solution to the NPR puzzle presented on Oct 21, 2018.

  A (paraphrased) statement of the problem:
    Given the letters in the word 'beermouth', contruct a three by three matrix of words where all
    the verticals, horizontals, and diagonals form three letter words.
*/

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;

// Gets the nth char from `word`.
fn get_char(word: &str, index: usize) -> char {
    word.chars().nth(index).unwrap()
}

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

// The middle letter has to be the middle letter of at least four words. Construct lists of
// possible sets of words.
fn shared_middle_letters(words: &[[char; 3]]) -> Vec<Vec<[char; 3]>> {
    let mut map = HashMap::new();
    for word in words {
        map.entry(word[1]).or_insert(Vec::new()).push(*word)
    }
    let mut out = Vec::new();
    for (_, val) in map.drain() {
        if val.len() >= 4 {
            out.push(val);
        }
    }
    out
}

// Checks that the first and last letters of `word` are still in `right`.
fn two_letter_less_than(word: &[char; 3], right: &HashMap<char, u8>) -> bool {
    for letter in [word[0], word[2]].iter() {
        match right.get(letter) {
            Some(count) => if *count <= 0 {
                return false;
            },
            _ => return false,
        }
    }
    true
}

// Checks if the word is a subset of the letters in `letter_map`. If so, it decrements the counts
// in `letter_map` by the first and third letters of the word. Ignores the middle letter since that
// is shared.
// Returns whether or not the word was a subset.
fn clear_two_letters(word: &[char; 3], letter_map: &mut HashMap<char, u8>) -> bool {
    if !two_letter_less_than(word, letter_map) {
        return false;
    }
    *letter_map.get_mut(&word[0]).unwrap() -= 1;
    *letter_map.get_mut(&word[2]).unwrap() -= 1;
    true
}

// If the word is a candidate, append it. Otherwise, do nothing. Returns whether or not the word
// was appended.
fn append_word_if<'a>(
    word: &[char; 3],
    letter_map: &mut HashMap<char, u8>,
    words: &mut Vec<[char; 3]>,
) -> bool {
    if !clear_two_letters(word, letter_map) {
        return false;
    }
    words.push(*word);
    true
}

fn four_word_product<'a>(words: &[[char; 3]], base_map: &HashMap<char, u8>) -> Vec<Vec<[char; 3]>> {
    let mut out = Vec::new();
    let middle_letter = words[0][1];
    for ii in 0..words.len() {
        let mut map = base_map.clone();
        let mut four_words = Vec::new();
        *map.get_mut(&middle_letter).unwrap() -= 1;
        if !append_word_if(&words[ii], &mut map, &mut four_words) {
            continue;
        }
        assert_eq!(four_words.len(), 1);
        for jj in ii + 1..words.len() {
            // Truncate here so that we only get one word from this pass.
            four_words.truncate(1);
            // Create a new map per iteration.
            let mut map = map.clone();
            if !append_word_if(&words[jj], &mut map, &mut four_words) {
                continue;
            }
            assert_eq!(four_words.len(), 2);
            for kk in jj + 1..words.len() {
                four_words.truncate(2);
                let mut map = map.clone();
                if !append_word_if(&words[kk], &mut map, &mut four_words) {
                    continue;
                }
                assert_eq!(four_words.len(), 3);
                for ll in kk + 1..words.len() {
                    four_words.truncate(3);
                    let mut map = map.clone();
                    if append_word_if(&words[ll], &mut map, &mut four_words) {
                        assert_eq!(four_words.len(), 4);
                        out.push(four_words.clone());
                    }
                }
            }
        }
    }
    out
}

// Find all of the combinations of four words that share a middle letter and the combined letters
// of all four words are still a subset of 'base_map'.
fn four_word_candidates<'a>(
    shared_middle: &[Vec<[char; 3]>],
    base_map: &HashMap<char, u8>,
) -> Vec<Vec<[char; 3]>> {
    let mut out = Vec::new();
    for words in shared_middle {
        out.append(&mut four_word_product(words, base_map));
    }
    out
}

// For four indices between zero and 4, return the missing index.
fn missing_index(ii: usize, jj: usize, kk: usize) -> usize {
    assert!(ii < 4);
    assert!(jj < 4);
    assert!(kk < 4);
    for ll in 0..4 {
        if ll != ii && ll != jj && ll != kk {
            return ll;
        }
    }
    unreachable!()
}

// Takes the candidates and finds words that make a proper word in the top row. The words are
// returned such that the first word is the left to right diagonal (1, 5, 9 on a phone), the second
// word is the middle vertical, the third is the right to left diagonal (3, 5, 7) and the remaining
// word is last.
fn top_row_candidates<'a>(
    four_cands: &Vec<Vec<[char; 3]>>,
    possible_words: &HashSet<[char; 3]>,
) -> Vec<Vec<[char; 3]>> {
    let mut out = Vec::new();
    for cand in four_cands {
        assert!(cand.len() == 4);
        for ii in 0..cand.len() {
            let first = cand[ii];
            for jj in 0..cand.len() {
                if jj == ii {
                    continue;
                }
                let second = cand[jj];
                for kk in 0..cand.len() {
                    if kk == jj || kk == ii {
                        continue;
                    }
                    let third = cand[kk];
                    let word = [first[0], second[0], third[0]];
                    if possible_words.contains(&word) {
                        let mut top_cand = vec![first, second, third];
                        top_cand.push(cand[missing_index(ii, jj, kk)]);
                        out.push(top_cand);
                    }
                }
            }
        }
    }
    out
}

// Filter out sets of four where the bottom generated from the top three words isn't a word.
fn filter_by_bottom<'a>(
    top_cands: Vec<Vec<[char; 3]>>,
    possible_words: &HashSet<[char; 3]>,
) -> Vec<Vec<[char; 3]>> {
    let mut out = Vec::new();
    for cand in top_cands {
        assert!(cand.len() == 4);
        let first = cand[0][2];
        let second = cand[1][2];
        let third = cand[2][2];
        let word = [first, second, third];
        if possible_words.contains(&word) {
            out.push(cand);
            continue;
        }
    }
    out
}

fn filter_by_rest<'a>(
    cands: Vec<Vec<[char; 3]>>,
    possible_words: &HashSet<[char; 3]>,
) -> Vec<Vec<[char; 3]>> {
    let mut out = Vec::new();
    for cand in cands {
        // The left vertical word is made up of the first letter of the first top word (left to
        // right diagnol), the first letter of the remaining word, and the last letter of the
        // third top word (right to left diagonal).
        let left_v_word = [cand[0][0], cand[3][0], cand[2][2]];
        if !possible_words.contains(&left_v_word) {
            continue;
        }
        // The right vertical word is from the first letter of the third top word, the third letter
        // of the remaining word and the third letter of the first top word.
        let right_v_word = [cand[2][0], cand[3][2], cand[0][2]];
        if !possible_words.contains(&right_v_word) {
            continue;
        }

        out.push(cand);
    }
    out
}

// Returns the 3x3 grid of letters in row major form.
fn presentation_format(cand: &[[char; 3]]) -> Vec<Vec<char>> {
    let mut out = Vec::new();
    // Fill in the vectors with x's just as a placeholder.
    for _ in 0..3 {
        out.push(vec!['x', 'x', 'x']);
    }
    // The first word is the left to right diagonal.
    out[0][0] = cand[0][0];
    out[1][1] = cand[0][1];
    out[2][2] = cand[0][2];
    // The second word is the middle vertical.
    out[0][1] = cand[1][0];
    out[2][1] = cand[1][2];
    // The third word is the left diagonal.
    out[0][2] = cand[2][0];
    out[2][0] = cand[2][2];
    // The last word is the middle horizontal.
    out[1][0] = cand[3][0];
    out[1][2] = cand[3][2];
    out
}

fn main() {
    let f = match File::open("dictionary.txt") {
        Ok(f) => f,
        Err(why) => panic!("Failed to open links.txt: {}", Error::description(&why)),
    };

    // Construct a vector of three letter words that are a subset of beermouth.
    let source_word = construct_letter_map("beermouth");
    let words: Vec<[char; 3]> = BufReader::new(f)
        .lines()
        .map(|s| s.unwrap())
        .filter(|s| s.len() == 3)
        .filter(|s| letter_map_less_than(&construct_letter_map(s), &source_word))
        .map(|s| [get_char(&s, 0), get_char(&s, 1), get_char(&s, 2)])
        .collect();

    // Find sets of words that share a middle letter.
    let middle_candidates = shared_middle_letters(&words);
    // Find sets of four words that share a middle letter and could make up the solution.
    let four_cands = four_word_candidates(&middle_candidates, &source_word);

    // Find possible top rows.
    let word_set = {
        let mut word_set = HashSet::with_capacity(words.len());
        for word in words.iter() {
            word_set.insert(word.clone());
        }
        word_set
    };
    let top_cands = top_row_candidates(&four_cands, &word_set);
    let bottom_cands = filter_by_bottom(top_cands, &word_set);
    let last_cands = filter_by_rest(bottom_cands, &word_set);

    for last_cand in last_cands {
        println!("Last candidate words:");
        for word in last_cand.iter() {
            let word_str: String = word.iter().collect();
            print!("{} ", word_str);
        }
        println!("\nIn grid format:");
        let present = presentation_format(&last_cand);
        for line in present {
            for ch in line {
                print!(" {} ", ch);
            }
            print!("\n");
        }
        print!("\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letter_map() {
        let mut expected = HashMap::new();
        expected.insert('b', 1);
        expected.insert('e', 2);
        expected.insert('r', 1);
        expected.insert('m', 1);
        expected.insert('o', 1);
        expected.insert('u', 1);
        expected.insert('t', 1);
        expected.insert('h', 1);
        assert_eq!(expected, construct_letter_map("beermouth"));
    }

    #[test]
    fn test_map_less_than() {
        let source_word = construct_letter_map("beermouth");
        assert!(letter_map_less_than(
            &construct_letter_map("bee"),
            &source_word
        ));
        assert!(letter_map_less_than(
            &construct_letter_map("bum"),
            &source_word
        ));
        assert!(letter_map_less_than(
            &construct_letter_map("but"),
            &source_word
        ));
        assert!(letter_map_less_than(
            &construct_letter_map("out"),
            &source_word
        ));
        assert!(!letter_map_less_than(
            &construct_letter_map("boo"),
            &source_word
        ));
        assert!(!letter_map_less_than(
            &construct_letter_map("oom"),
            &source_word
        ));
        assert!(!letter_map_less_than(
            &construct_letter_map("ouf"),
            &source_word
        ));
    }

    #[test]
    fn test_shared_middle() {
        let mut words = Vec::new();
        words.push(['b', 'e', 'e']);
        words.push(['b', 'e', 't']);
        words.push(['m', 'e', 't']);
        words.push(['b', 'u', 'm']);
        words.push(['b', 'u', 't']);
        words.push(['o', 'u', 't']);
        words.push(['o', 'u', 'f']);
        words.push(['b', 'o', 't']);
        words.push(['h', 'o', 't']);
        words.push(['m', 'o', 't']);
        words.push(['t', 'o', 'm']);
        words.push(['r', 'o', 't']);
        let mut expected = Vec::new();
        {
            let mut group = Vec::new();
            for ii in 3..7 {
                group.push(words[ii]);
            }
            group.sort();
            expected.push(group);
        }
        {
            let mut group = Vec::new();
            for ii in 7..12 {
                group.push(words[ii]);
            }
            group.sort();
            expected.push(group);
        }
        let mut input_words = words.clone();
        input_words.sort();
        let mut shared = shared_middle_letters(&input_words);
        // Sort by the lengths of the sub-Vecs to prevent test flakes.
        shared.sort_by(|l, r| l.len().cmp(&r.len()));
        assert_eq!(expected, shared);
    }
}
