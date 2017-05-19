use std::env;
use std::io::prelude::*;
use std::io::{BufReader, Lines};
use std::fs::File;
use std::path::Path;
use std::collections::{HashMap, HashSet};


fn read_stop_words(stop_words_file: &Path) -> Result<HashSet<String>, std::io::Error> {
    BufReader::new(File::open(stop_words_file)?)
        .split(b',')
        .map(|b| Ok(String::from_utf8_lossy(&b?).into())).collect()
}


fn count_words(stop_words: &HashSet<String>, lines: Lines<BufReader<File>>) -> Result<Vec<(String, i64)>, std::io::Error> {
    let mut count_map = HashMap::new();

    for line_result in lines {
        let line = line_result?;
        let words = line.split(|c: char| !c.is_alphanumeric())
            .filter(|word| word.len() >= 2)
            .map(|word| word.to_lowercase())
            .filter(|word| !stop_words.contains(word));

        for word in words {
            *count_map.entry(word).or_insert(0) += 1;
        }
    }

    Ok(count_map.into_iter().collect())
}


fn main_internal(text_file: &Path) -> Result<(), std::io::Error> {
    let stop_words = read_stop_words(Path::new("test-data/stop_words.txt"))?;

    let mut counts = count_words(&stop_words, BufReader::new(File::open(text_file)?).lines())?;
    counts.sort_by_key(|c| -c.1);

    for (w, c) in counts.into_iter().take(25) {
        println!("{}  -  {}", w, c);
    }

    Ok(())
}


fn main() {
    if let Some(file_name) = env::args().nth(1) {
        if let Err(err) = main_internal(Path::new(&file_name)) {
            println!("Io Error: {:?}", err);
        }
    } else {
        println!("Please provide a filename as first argument!");
    }
}
