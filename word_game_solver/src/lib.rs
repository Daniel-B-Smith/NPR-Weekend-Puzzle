use std::vec::Vec;

use std::collections::{HashMap, HashSet};

use rand::{thread_rng, Rng};

pub fn is_unique(s: &str) -> bool {
    let set: HashSet<char> = s.chars().collect();
    set.len() == s.len()
}

pub fn letter_combos(words: &[String]) -> HashMap<Vec<char>, Vec<&String>> {
    let mut combos = HashMap::new();
    for word in words {
        let mut chars: Vec<char> = word.chars().collect();
        chars.sort();
        combos.entry(chars).or_insert(Vec::new()).push(word);
    }
    combos
}

pub fn gen_guess<'a>(combos: &HashMap<Vec<char>, Vec<&'a String>>) -> &'a String {
    let mut rng = thread_rng();

    let size = combos.len();
    let select = rng.gen_range(0, size);
    let words = combos.iter().nth(select).unwrap().1;
    words[rng.gen_range(0, words.len())]
}

fn num_matches(first: &[char], second: &[char]) -> i32 {
    let mut matches = 0;
    for c in first {
        if second.contains(&c) {
            matches += 1;
        }
    }
    matches
}

pub fn filter_matches(word: &str, matching: i32, combos: &mut HashMap<Vec<char>, Vec<&String>>) {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort();

    combos.retain(|k, _| num_matches(k, &chars) == matching);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_unique() {
        assert!(is_unique("abc"));
        assert!(!is_unique("aabc"));
    }

    #[test]
    fn test_letter_combos() {
        let head = "head".to_string();
        let them = "them".to_string();
        let meth = "meth".to_string();
        let mut expected = HashMap::new();
        expected.insert(vec!['a', 'd', 'e', 'h'], vec![&head]);
        expected.insert(vec!['e', 'h', 'm', 't'], vec![&them, &meth]);

        assert_eq!(
            letter_combos(&[them.to_string(), head.to_string(), meth.to_string()]),
            expected
        );
    }

    #[test]
    fn test_gen_guess() {
        let head = "head".to_string();
        let them = "them".to_string();
        let meth = "meth".to_string();
        let mut source = HashMap::new();
        source.insert(vec!['a', 'd', 'e', 'h'], vec![&head]);
        source.insert(vec!['e', 'h', 'm', 't'], vec![&them, &meth]);

        let mut words = HashSet::new();
        words.insert(&head);
        words.insert(&them);
        words.insert(&meth);

        // Make a bunch of guesses.
        for _ in 1..10 {
            let guess = gen_guess(&source);
            assert!(words.contains(guess));
        }
    }

    #[test]
    fn test_num_matches() {
        assert_eq!(num_matches(&['a', 'b', 'c'], &['d', 'e', 'f']), 0);
        assert_eq!(num_matches(&['a', 'b', 'd'], &['d', 'e', 'f']), 1);
        assert_eq!(num_matches(&['a', 'd', 'g'], &['d', 'e', 'f']), 1);
        assert_eq!(num_matches(&['a', 'd', 'e'], &['d', 'e', 'f']), 2);
        assert_eq!(num_matches(&['a', 'd', 'f'], &['d', 'e', 'f']), 2);
        assert_eq!(num_matches(&['d', 'f', 'g'], &['d', 'e', 'f']), 2);
        assert_eq!(num_matches(&['a', 'b', 'c'], &['a', 'b', 'c']), 3);
    }

    #[test]
    fn test_filter_matches() {
        let head = "head".to_string();
        let them = "them".to_string();
        let meth = "meth".to_string();
        let mut source = HashMap::new();
        source.insert(vec!['a', 'd', 'e', 'h'], vec![&head]);
        source.insert(vec!['e', 'h', 'm', 't'], vec![&them, &meth]);

        // 'math' with three matches should only filter 'head'.
        filter_matches("math", 3, &mut source);

        let mut expected = HashMap::new();
        expected.insert(vec!['e', 'h', 'm', 't'], vec![&them, &meth]);

        assert_eq!(source, expected);
    }
}
