fn as_sorted_string(word : &str) -> String {
    let mut sorted : Vec<char> = word.chars().collect();
    sorted.sort();

    let mut sorted_string = String::with_capacity(sorted.len());
    for letter in sorted {
        sorted_string.push(letter);
    }
    return sorted_string.to_lowercase();
}

pub fn anagrams_for<'a>(word : &str, candidates : &[&'a str]) -> Vec<&'a str> {
    let mut anagrams : Vec<&str> = vec![];
    let lower_word = word.to_lowercase();
    let sorted = as_sorted_string(&lower_word);
    
    for candidate in candidates {
        if candidate.len() == sorted.len() {
            let lower_candidate = candidate.to_lowercase();
            if lower_word != lower_candidate {
                let sorted_candidate = as_sorted_string(&lower_candidate);
                println!("{} {} {}", sorted, candidate, sorted_candidate);
                if sorted_candidate == sorted  {
                    anagrams.push(candidate);
                }
            }
        }
    }
    return anagrams;
}
