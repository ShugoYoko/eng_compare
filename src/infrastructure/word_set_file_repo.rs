use crate::domain::word_set;
use std::fs;
use std::io::{BufRead, BufReader};

pub struct WordSetFileRepo {
    pub folder_path: String,
}

impl WordSetFileRepo {
    pub fn new(path: &str) -> Self {
        Self {
            folder_path: path.to_string(),
        }
    }

    pub fn get_word_set(&self) -> word_set::WordSet {
        let mut word_set = word_set::WordSet::new();
        let path = self.folder_path.as_str();
        let entries = fs::read_dir(path).unwrap();

        for entry in entries {
            let path = entry.unwrap().path();
            let input = fs::File::open(path).unwrap();
            let buffered = BufReader::new(input);

            for line in buffered.lines() {
                word_set.add(line.unwrap().as_str());
            }
        }
        word_set
    }
}
