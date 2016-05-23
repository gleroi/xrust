use std::collections::HashMap;
use std::str::Chars;

struct SplitWords<'a> {
    phrase: &'a str,
    current_position: usize,
    current_char: Option<char>,
    iterator: Chars<'a>
}

impl <'a> SplitWords<'a> {
    fn is_word_separator(letter : char) -> bool {
        letter.is_whitespace() || (!letter.is_alphabetic() && !letter.is_numeric())
    }

    fn next_char(&mut self) {
        self.current_char = self.iterator.next();
        self.current_position += 1;
    }
    
    fn current_char(&self) -> Option<char> {
        return self.current_char;
    }
}

fn split_words(phrase: &str) -> SplitWords {
    let mut split = SplitWords {
        phrase: phrase,
        current_position: 0,
        current_char: None,
        iterator: phrase.chars(),
    };
    split.current_char = split.iterator.next();
    return split;
}

impl <'a> Iterator for SplitWords<'a> {
    type Item = &'a str;
    
    fn next(&mut self) -> Option<Self::Item> {
        let phrase_len = self.phrase.len();
        if self.current_position >= phrase_len {
            return None;
        }
        while let Some(letter) = self.current_char() {
            if SplitWords::is_word_separator(letter) {
                self.next_char();
            }
            else {
                break;
            }
        }
        println!("current_position: {}", self.current_position);
        let start : usize = self.current_position;
        while let Some(letter) = self.current_char()  {
            if !SplitWords::is_word_separator(letter) {
                self.next_char();
            }
            else {
                break;
            }
        }

        if self.current_position > start {
            let word = &self.phrase[start..(self.current_position)];
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

#[allow(dead_code)]
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

