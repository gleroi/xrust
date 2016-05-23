use std::collections::HashMap;
use std::str::Chars;

struct SplitWords<'a> {
    phrase: &'a str,
    current_position: usize,
}

impl <'a> SplitWords<'a> {
    fn is_word_separator(letter : char) -> bool {
        letter.is_whitespace() || (!letter.is_alphabetic() && !letter.is_numeric())
    }
}

fn split_words(phrase: &str) -> SplitWords {
    return SplitWords {
        phrase: phrase,
        current_position: 0,
    };
}

impl <'a> Iterator for SplitWords<'a> {
    type Item = &'a str;
    
    fn next(&mut self) -> Option<Self::Item> {
        let phrase_len = self.phrase.len();
        if self.current_position >= phrase_len {
            return None;
        }
        println!("current_position: {}", self.current_position);
        while let Some(letter) = self.current.next() {
            self.current_position += 1;
            if !SplitWords::is_word_separator(letter) {
                break;
            }
        }
        println!("current_position: {}", self.current_position);
        let start : usize = self.current_position;
        while let Some(letter) = self.current.next() {
            self.current_position += 1;
            println!("current_position: {}, letter: {}", self.current_position, letter);
            if SplitWords::is_word_separator(letter) {
                break;
            }
        }
        println!("start: {}, current_position: {}", start, self.current_position);
        if self.current_position > start {
            let word = &self.phrase[start..(self.current_position)];
            println!("found word: {},", word);
            return Some(word);
        }
        return None;
    }
}

pub fn word_count(phrase: &str) -> HashMap<String, u32> {
    let icase_phrase = phrase.to_lowercase();
    let mut counts: HashMap<String, u32> = HashMap::new();
    
    for word in split_words(&icase_phrase) {
        let count = counts.entry(word.to_string()).or_insert(0);
        *count += 1;
    }
    return counts;
}

/*
mod using_split_iterator {
    
    use std::collections::HashMap;

    fn is_word_separator(letter : char) -> bool {
        letter.is_whitespace() || (!letter.is_alphabetic() && !letter.is_numeric())
    }

    fn is_valid_word(word: &str) -> bool {
        word.len() > 0
    }
    
    pub fn word_count(phrase: &str) -> HashMap<String, u32> {
        let icase_phrase = phrase.to_lowercase();
        let mut counts: HashMap<String, u32> = HashMap::new();
        
        for word in icase_phrase.split(is_word_separator) {
            if is_valid_word(word) {
                let count = counts.entry(word.to_string()).or_insert(0);
                *count += 1;
            }
        }
        return counts;
    }
}
*/
