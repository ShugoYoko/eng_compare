use regex::Regex;

#[derive(PartialEq, Eq, Hash)]
pub struct Word {
    pub contents: String,
}

impl Word {
    pub fn new(text: &str) -> Self {
        let word = text.to_lowercase();
        Self { contents: word }
    }

    pub fn can_add(&self) -> bool {
        let re = Regex::new(r"\d+").unwrap();
        if re.is_match(self.contents.as_str())
            || self.contents.contains("'")
            || self.contents.contains("’")
        {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::word::Word;

    #[test]
    fn word_works() {
        let word1 = Word::new("ABC");
        assert_eq!(word1.contents, "abc".to_string());
        assert_eq!(word1.can_add(), true);

        let word2 = Word::new("A1");
        assert_eq!(word2.contents, "a1".to_string());
        assert_eq!(word2.can_add(), false);

        let word3 = Word::new("yoU'll");
        assert_eq!(word3.contents, "you'll".to_string());
        assert_eq!(word3.can_add(), false);
    }
}
