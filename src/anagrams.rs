use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, inputs: &[&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let mut word_chars: Vec<char> = word_lower.chars().collect();
    word_chars.sort();
    let word_sorted: String = word_chars.into_iter().collect();

    inputs
        .iter()
        .filter(|&&input| {
            let input_lower = input.to_lowercase();
            let mut input_chars: Vec<char> = input_lower.chars().collect();
            input_chars.sort();
            let input_sorted: String = input_chars.into_iter().collect();

            word_sorted == input_sorted && input_lower != word_lower
        })
        .copied()
        .collect()
}
