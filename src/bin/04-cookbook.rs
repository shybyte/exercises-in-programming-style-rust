use std::io::prelude::*;
use std::env;
use std::io::{BufReader};
use std::fs::File;
use std::collections::{HashSet};
use std::cell::RefCell;

fn main() {
    // The shared mutable data
    let data: RefCell<Vec<char>> = RefCell::new(vec![]);
    let words: RefCell<Vec<String>> = RefCell::new(vec![]);
    let word_freqs: RefCell<Vec<(String, i64)>> = RefCell::new(vec![]);

    // The procedures

    /// Takes a path to a file and assigns the entire
    /// contents of the file to the global variable data
    let read_file = |filename: &str| {
        let mut text = String::new();
        File::open(filename).unwrap().read_to_string(&mut text).unwrap();
        data.borrow_mut().append(&mut text.chars().collect());
    };


    /// Replaces all nonalphanumeric chars in data with white space
    let filter_chars_and_normalize = || {
        for c in data.borrow_mut().iter_mut() {
            if !c.is_alphanumeric() {
                *c = ' ';
            } else {
                *c = c.to_lowercase().next().unwrap();
            }
        }
    };

    /// Scans data for words, filling the global variable words
    let scan = || {
        let data_str: String = data.borrow().iter().collect();
        words.borrow_mut().append(&mut data_str.split(' ').map(|s| s.to_string()).filter(|s| !s.is_empty()).collect());
    };

    /// Scans data for words, filling the global variable words
    let remove_stop_words = || {
        let mut stop_words: HashSet<String> = BufReader::new(File::open("test-data/stop_words.txt").unwrap())
            .split(b',').map(|b| String::from_utf8_lossy(&b.unwrap()).into()).collect();
        for ascii_lowercase in b'a'..b'z' { stop_words.insert((ascii_lowercase as char).to_string()); }

        let mut indexes = vec![];

        for (i, word) in words.borrow().iter().enumerate() {
            if stop_words.contains(word) {
                indexes.push(i)
            }
        }

        for i in indexes.into_iter().rev() {
            words.borrow_mut().remove(i);
        }
    };

    /// Creates a list of pairs associating words with frequencies
    let frequencies = || {
        let mut word_freqs_mut = word_freqs.borrow_mut();
        for w in words.borrow().iter() {
            if let Some(index) = word_freqs_mut.iter().position(|pair| &pair.0 == w) {
                word_freqs_mut[index].1 += 1;
            } else {
                word_freqs_mut.push((w.to_string(), 1))
            }
        }
    };

    /// Sorts word_freqs by frequency
    let sort = || {
        word_freqs.borrow_mut().sort_by_key(|p| 0 - p.1);
    };


    read_file(&env::args().nth(1).unwrap());
    filter_chars_and_normalize();
    scan();
    remove_stop_words();
    frequencies();
    sort();

    for &(ref w, ref c) in word_freqs.borrow().iter().take(25) {
        println!("{}  -  {}", w, c);
    }
}
