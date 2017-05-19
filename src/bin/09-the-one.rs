//#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]

use std::io::prelude::*;
use std::env;
use std::char;
use std::io::{BufReader};
use std::fs::File;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;


struct TFTheOne<T> {
    value: T
}

impl<T> TFTheOne<T> {
    pub fn new(v: T) -> Self {
        TFTheOne { value: v }
    }

    pub fn bind<R>(self, func: fn(T) -> R) -> TFTheOne<R> {
        TFTheOne::new(func(self.value))
    }

    fn bind_ref<R>(self, func: fn(&T) -> R) -> TFTheOne<R> {
        TFTheOne::new(func(&self.value))
    }

    fn bind_as_ref<R, TR>(self, func: fn(&TR) -> R) -> TFTheOne<R>
        where T: AsRef<TR>, TR: ? Sized
    {
        TFTheOne::new(func(self.value.as_ref()))
    }
}

trait PrintMe {
    fn printme(&self);
}

impl<T: Display> PrintMe for TFTheOne<T> {
    fn printme(&self) {
        print!("{}", self.value);
    }
}


fn read_file(path_to_file: &str) -> String {
    let mut text = String::new();
    File::open(path_to_file).unwrap().read_to_string(&mut text).unwrap();
    text
}

fn filter_chars_and_normalize(str_data: &str) -> String {
    str_data.replace(|c: char| !c.is_alphanumeric(), " ").to_lowercase()
}

fn scan(str_data: &str) -> Vec<String> {
    str_data.split(|c: char| !c.is_alphanumeric())
        .filter(|&word| word.len() >= 1)
        .map(|w| w.to_string()).collect()
}

fn remove_stop_words(word_list: &[String]) -> Vec<String> {
    let mut stop_words: HashSet<String> = BufReader::new(File::open("test-data/stop_words.txt").unwrap())
        .split(b',').map(|b| String::from_utf8_lossy(&b.unwrap()).into()).collect();
    for ascii_lowercase in b'a'..b'z' { stop_words.insert((ascii_lowercase as char).to_string()); }

    word_list.iter().filter(|&word| !stop_words.contains(word)).map(|w| w.to_string()).collect()
}

fn frequencies(word_list: Vec<String>) -> HashMap<String, i64> {
    let mut word_freqs: HashMap<String, i64> = HashMap::new();
    for w in word_list {
        *word_freqs.entry(w).or_insert(0) += 1;
    }
    word_freqs
}

fn sort(word_freq: &HashMap<String, i64>) -> Vec<(String, i64)> {
    let mut pairs: Vec<_> = word_freq.clone().into_iter().collect();
    pairs.sort_by_key(|c| -c.1);
    pairs
}


fn top25_freqs(word_freqs: &[(String, i64)]) -> String {
    let mut top25 = String::new();
    for tf in word_freqs.iter().take(25) {
        top25 += &format!("{}  -  {}\n", tf.0.as_str(), tf.1);
    }
    top25
}

fn main() {
    TFTheOne::new(env::args().nth(1).unwrap())
        .bind_as_ref(read_file)
        .bind_as_ref(filter_chars_and_normalize)
        .bind_as_ref(scan)
        .bind_as_ref(remove_stop_words)
        .bind(frequencies)
        .bind_ref(sort)
        .bind_as_ref(top25_freqs)
        .printme();
}
