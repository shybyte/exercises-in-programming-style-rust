use std::io::prelude::*;
use std::env;
use std::io::{BufReader};
use std::fs::File;
use std::collections::{HashMap, HashSet};

fn main() {
    let stop_words: HashSet<String> = BufReader::new(File::open("test-data/stop_words.txt").unwrap())
        .split(b',').map(|b| String::from_utf8_lossy(&b.unwrap()).into()).collect();

    let mut text = String::new();
    File::open(env::args().nth(1).unwrap()).unwrap().read_to_string(&mut text).unwrap();

    let mut counts: Vec<_> = text.to_lowercase().split(|c: char| !c.is_alphanumeric())
        .filter(|&word| word.len() >= 2 && !stop_words.contains(word))
        .fold(HashMap::new(), |mut map, word| {
            *map.entry(word.to_string()).or_insert(0) += 1;
            map
        }).into_iter().collect();

    counts.sort_by_key(|c| -c.1);

    for (w, c) in counts.into_iter().take(25) {
        println!("{}  -  {}", w, c);
    }
}
