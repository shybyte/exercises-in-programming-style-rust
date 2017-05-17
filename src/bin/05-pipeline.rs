use std::io::prelude::*;
use std::env;
use std::char;
use std::io::{BufReader};
use std::fs::File;
use std::collections::{HashMap, HashSet};
use std::cmp;

/// Takes a path to a file and returns the entire
/// contents of the file as a string
fn read_file(path_to_file: &str) -> String {
    let mut text = String::new();
    File::open(path_to_file).unwrap().read_to_string(&mut text).unwrap();
    text
}

/// Takes a string and returns a copy with all nonalphanumeric
/// chars replaced by white space
fn filter_chars_and_normalize(str_data: &str) -> String {
    str_data.replace(|c: char| !c.is_alphanumeric(), " ").to_lowercase()
}

/// Takes a string and scans for words, returning
/// a list of words.
fn scan(str_data: &str) -> Vec<String> {
    str_data.split(|c: char| !c.is_alphanumeric())
        .filter(|&word| word.len() >= 1)
        .map(|w| w.to_string()).collect()
}

/// Takes a list of words and returns a copy with all stop
/// words removed
fn remove_stop_words(word_list: &[String]) -> Vec<String> {
    let mut stop_words: HashSet<String> = BufReader::new(File::open("test-data/stop_words.txt").unwrap())
        .split(b',').map(|b| String::from_utf8_lossy(&b.unwrap()).into()).collect();
    for ascii_lowercase in b'a'..b'z' { stop_words.insert((ascii_lowercase as char).to_string()); }

    word_list.iter().filter(|&word| !stop_words.contains(word)).map(|w| w.to_string()).collect()
}

/// Takes a list of words and returns a dictionary associating
/// words with frequencies of occurrence
fn frequencies(word_list: &[String]) -> HashMap<String, i64> {
    let mut word_freqs: HashMap<String, i64> = HashMap::new();
    for w in word_list {
        if word_freqs.contains_key(w) {
            *word_freqs.get_mut(w).unwrap() += 1;
        } else {
            word_freqs.insert(w.clone(), 1);
        }
    }
    word_freqs
}


/// Takes a dictionary of words and their frequencies
/// and returns a list of pairs where the entries are
/// sorted by frequency
fn sort(word_freq: &HashMap<String, i64>) -> Vec<(String, i64)> {
    let mut pairs: Vec<_> = word_freq.clone().into_iter().collect();
    pairs.sort_by_key(|c| -c.1);
    pairs
}


/// Takes a list of pairs where the entries are sorted by frequency and prints them recursively.
fn print_all(word_freqs: &[(String, i64)]) {
    if !word_freqs.is_empty() {
        println!("{}  -  {}", word_freqs[0].0, word_freqs[0].1);
        print_all(&word_freqs[1..]);
    }
}

fn main() {
    let word_freqs = sort(&frequencies(&remove_stop_words(&scan(&filter_chars_and_normalize(&read_file(&env::args().nth(1).unwrap()))))));
    print_all(&word_freqs[0..cmp::min(word_freqs.len(), 25)]);
}
