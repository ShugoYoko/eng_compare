use std::collections::HashSet;

use super::word::Word;

pub struct WordSet {
    pub words: HashSet<Word>,
}

impl WordSet {
    pub fn new() -> Self {
        let list = HashSet::new();
        Self { words: list }
    }

    pub fn get_count(&self) -> i32 {
        let count = self.words.len() as i32;
        count
    }

    pub fn add(&mut self, context: &str) {
        let add_word: Vec<&str> = context
            .split(|c| c == ',' || c == ':' || c == '!' || c == '?' || c == '.' || c == '/')
            .flat_map(|s| s.split_whitespace())
            .collect();

        for v in add_word.iter() {
            let word = Word::new(v);
            if word.can_add() && word.contents != "" {
                self.words.insert(word);
            }
        }
    }

    pub fn return_match(&self, ws: &WordSet) -> HashSet<Word> {
        let mut result: HashSet<Word> = HashSet::new();
        let inter = self.words.intersection(&ws.words);
        for word in inter {
            let add_word = Word::new(word.contents.as_str());
            result.insert(add_word);
        }
        result
    }

    pub fn return_nonmatch(&self, ws: &WordSet) -> HashSet<Word> {
        let mut result: HashSet<Word> = HashSet::new();
        let differ = self.words.difference(&ws.words);
        for word in differ {
            let add_word = Word::new(word.contents.as_str());
            result.insert(add_word);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::word_set::WordSet;

    #[test]
    fn wordset_works() {
        let target = "Mom, do you know where my phone is?is 5ab";
        let mut wordset = WordSet::new();
        wordset.add(target);
        assert_eq!(wordset.get_count(), 8);
    }

    #[test]
    fn set_works() {
        let target = "Mom, do you know where my phone is?is 5ab";
        let mut target_set = WordSet::new();
        target_set.add(target);

        let data = "mom,do you have A pen?";
        let mut dataset = WordSet::new();
        dataset.add(data);

        //積集合
        let inter = target_set.return_match(&dataset);
        assert_eq!(inter.len(), 3);

        //差集合
        let diff = target_set.return_nonmatch(&dataset);
        assert_eq!(diff.len(), 5);
    }
}
