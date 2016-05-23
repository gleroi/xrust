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
