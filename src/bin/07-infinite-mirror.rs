use std::io::prelude::*;
use std::env;
use std::io::{BufReader};
use std::fs::File;
use std::collections::{HashMap, HashSet};
use std::cmp;

// Mileage may vary. If this crashes, make it lower
const RECURSION_LIMIT: usize = 9500;

fn count(word_list: &[String], stop_words: &HashSet<String>, word_freqs: &mut HashMap<String, i64>) {
    if let Some((word, remaining_words)) = word_list.split_first() {
        *word_freqs.entry(word.to_string()).or_insert(0) += 1;
        count(remaining_words, stop_words, word_freqs);
    }
}

fn wf_print(word_freq_pairs: &[(&String, &i64)]) {
    if let Some((first, remaining)) = word_freq_pairs.split_first() {
        println!("{}  -  {}", first.0, first.1);
        wf_print(remaining);
    }
}

fn main() {
    let stop_words: HashSet<String> = BufReader::new(File::open("test-data/stop_words.txt").unwrap())
        .split(b',').map(|b| String::from_utf8_lossy(&b.unwrap()).into()).collect();

    let mut text = String::new();
    File::open(env::args().nth(1).unwrap()).unwrap().read_to_string(&mut text).unwrap();

    let words: Vec<String> = text.to_lowercase().split(|c: char| !c.is_alphanumeric())
        .filter(|&word| word.len() >= 2 && !stop_words.contains(word))
        .map(|s| s.to_string())
        .collect();

    let mut word_freqs = HashMap::new();
    // Theoretically, we would just call count(&words[..], &stop_words, &mut word_freqs)
    // Try doing that and see what happens.
    for chunk in words.chunks(RECURSION_LIMIT) {
        count(chunk, &stop_words, &mut word_freqs);
    }

    let mut word_freq_pairs: Vec<_> =  word_freqs.iter().collect();
    word_freq_pairs.sort_by_key(|c| -c.1);
    wf_print(&word_freq_pairs[0..cmp::min(word_freq_pairs.len(), 25)])
}
