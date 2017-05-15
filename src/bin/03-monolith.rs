use std::io::prelude::*;
use std::env;
use std::io::{BufReader};
use std::fs::File;
use std::collections::{HashSet};

fn main() {
    // the global list of (word, frequency) pairs
    let mut word_freqs: Vec<(String, usize)> = vec![];

    // the list of stop words
    let mut stop_words: HashSet<String> = BufReader::new(File::open("test-data/stop_words.txt").unwrap())
        .split(b',').map(|b| String::from_utf8_lossy(&b.unwrap()).into()).collect();
    for ascii_lowercase in b'a'..b'z' { stop_words.insert((ascii_lowercase as char).to_string()); }

    // iterate through the file one line at a time
    for line_result in BufReader::new(File::open(env::args().nth(1).unwrap()).unwrap()).lines() {
        let line = line_result.unwrap() + "\n";
        let mut start_char_option: Option<usize> = None;
        let mut i = 0;
        let chars: Vec<_> = line.chars().collect();
        for c in &chars {
            if let Some(start_char) = start_char_option {
                if !c.is_alphanumeric() {
                    // We found the end of a word. Process it
                    let mut found = false;
                    let word = chars[start_char..i].iter().collect::<String>().to_lowercase();
                    // Ignore stop words
                    if !stop_words.contains(&word) {
                        let mut pair_index = 0;
                        for ref mut pair in &mut word_freqs {
                            if word == pair.0 {
                                found = true;
                                pair.1 += 1;
                                break;
                            }
                            pair_index += 1;
                        }
                        if !found {
                            word_freqs.push((word, 1));
                        } else if word_freqs.len() > 1 {
                            // We may need to reorder
                            for n in (0..pair_index).rev() {
                                if word_freqs[pair_index].1 > word_freqs[n].1 {
                                    word_freqs.swap(pair_index, n);
                                    pair_index = n
                                }
                            }
                        }
                    }
                    // Let's reset
                    start_char_option = None
                }
            } else if c.is_alphanumeric() {
                // We found the start of a word
                start_char_option = Some(i)
            }
            i += 1;
        }
    }

    for (w, c) in word_freqs.into_iter().take(25) {
        println!("{}  -  {}", w, c);
    }
}
