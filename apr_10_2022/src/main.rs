use std::fs::File;
use std::io::{BufRead, BufReader};

fn has_insert_front(five: &[u8], six: &[u8]) -> bool {
    assert_eq!(five.len(), 5);
    assert_eq!(six.len(), 6);
    five == &six[1..]
}

fn has_insert_five(six: &[u8], seven: &[u8]) -> bool {
    assert_eq!(six.len(), 6);
    assert_eq!(seven.len(), 7);

    if &six[..4] != &seven[..4] {
        return false;
    }

    &six[4..] == &seven[5..]
}

fn main() {
    let f = match File::open("dictionary.txt") {
        Ok(f) => f,
        Err(why) => panic!("Failed to open dictionary.txt: {}", why),
    };

    let mut fives = Vec::new();
    let mut sixes = Vec::new();
    let mut sevens = Vec::new();
    for word in BufReader::new(f).lines().map(|s| s.unwrap()) {
        let word = word.into_bytes();
        if !word.contains(&b'l') {
            continue;
        }
        match word.len() {
            5 => fives.push(word),
            6 => sixes.push(word),
            7 => sevens.push(word),
            _ => (),
        }
    }

    let mut pairs = Vec::new();
    for five in &fives {
        for six in &sixes {
            if has_insert_front(five, six) {
                pairs.push((five, six));
            }
        }
    }

    println!("Pairs: {}", pairs.len());
    /*println!(
            "Pair 1: {} {}",
            std::str::from_utf8(&pairs[0].0).unwrap(),
            std::str::from_utf8(&pairs[0].1).unwrap()
    );*/

    let mut triples = Vec::new();

    for pair in &pairs {
        for seven in &sevens {
            if has_insert_five(pair.1, seven) {
                triples.push((pair.0, pair.1, seven));
            }
        }
    }

    println!("Triples: {}", triples.len());
    for triple in &triples {
        println!(
            "{} {} {}",
            std::str::from_utf8(triple.0).unwrap(),
            std::str::from_utf8(triple.1).unwrap(),
            std::str::from_utf8(triple.2).unwrap()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_insert_front() {
        fn help_first(five: &str, six: &str) -> bool {
            has_insert_front(five.as_bytes(), six.as_bytes())
        }

        assert!(help_first("reach", "breach"));
        assert!(help_first("creak", "ccreak"));
        assert!(!help_first("creak", "creaks"));
        assert!(!help_first("crept", "accept"));
    }

    #[test]
    fn test_has_insert_five() {
        fn help_five(six: &str, seven: &str) -> bool {
            has_insert_five(six.as_bytes(), seven.as_bytes())
        }

        assert!(help_five("abcdef", "abcdgef"));
        assert!(!help_five("creakk", "creakss"));
    }
}
