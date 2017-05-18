use std::io::prelude::*;
use std::env;
use std::char;
use std::io::{BufReader};
use std::fs::File;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;


fn info<T: Debug>(object: &T) -> String {
    format!("{:?}", object)
}

trait TFExercise: Debug {
    fn info(&self) -> String {
        info(&self)
    }
}


#[derive(Debug)]
struct DataStorageManager {
    data: String
}

impl TFExercise for DataStorageManager {
    fn info(&self) -> String {
        info(self) + ": My major data structure is a String"
    }
}

impl DataStorageManager {
    pub fn new(path_to_file: &str) -> Self {
        let mut data = String::new();
        File::open(path_to_file).unwrap().read_to_string(&mut data).unwrap();
        data = data.replace(|c: char| !c.is_alphanumeric(), " ").to_lowercase();
        DataStorageManager { data }
    }

    pub fn words(&self) -> Vec<String> {
        self.data.split(|c: char| !c.is_alphanumeric())
            .filter(|&word| word.len() >= 1)
            .map(|w| w.to_string()).collect()
    }
}


#[derive(Debug)]
struct StopWordManager {
    stop_words: HashSet<String>
}

impl TFExercise for StopWordManager {
    fn info(&self) -> String {
        info(self) + ": My major data structure is a HashSet"
    }
}

impl StopWordManager {
    pub fn new() -> Self {
        let mut stop_words: HashSet<String> = BufReader::new(File::open("test-data/stop_words.txt").unwrap())
            .split(b',').map(|b| String::from_utf8_lossy(&b.unwrap()).into()).collect();
        for ascii_lowercase in b'a'..b'z' { stop_words.insert((ascii_lowercase as char).to_string()); }
        StopWordManager { stop_words }
    }

    pub fn is_stop_word(&self, word: &str) -> bool {
        self.stop_words.contains(word)
    }
}


#[derive(Debug)]
struct WordFrequencyManager {
    word_freqs: HashMap<String, i64>
}

impl TFExercise for WordFrequencyManager {
    fn info(&self) -> String {
        info(self) + ": My major data structure is a HashMap"
    }
}

impl WordFrequencyManager {
    pub fn new() -> Self {
        WordFrequencyManager { word_freqs: HashMap::new() }
    }

    pub fn increment_count(&mut self, word: &str) {
        if self.word_freqs.contains_key(word) {
            *self.word_freqs.get_mut(word).unwrap() += 1;
        } else {
            self.word_freqs.insert(word.to_string(), 1);
        }
    }

    pub fn sorted(&self) -> Vec<(String, i64)> {
        let mut pairs: Vec<_> = self.word_freqs.clone().into_iter().collect();
        pairs.sort_by_key(|c| -c.1);
        pairs
    }
}


#[derive(Debug)]
struct WordFrequencyController {
    storage_manager: DataStorageManager,
    stop_word_manager: StopWordManager,
    word_freq_manager: WordFrequencyManager
}

impl TFExercise for WordFrequencyController {}

impl WordFrequencyController {
    pub fn new(path_to_file: &str) -> Self {
        WordFrequencyController {
            storage_manager: DataStorageManager::new(path_to_file),
            stop_word_manager: StopWordManager::new(),
            word_freq_manager: WordFrequencyManager::new()
        }
    }

    pub fn run(&mut self) {
        for w in self.storage_manager.words() {
            if !self.stop_word_manager.is_stop_word(&w) {
                self.word_freq_manager.increment_count(&w);
            }
        }

        let word_freqs = self.word_freq_manager.sorted();
        for (w, c) in word_freqs.into_iter().take(25) {
            println!("{}  -  {}", w, c);
        }
    }
}


fn main() {
    WordFrequencyController::new(&env::args().nth(1).unwrap()).run();
}
