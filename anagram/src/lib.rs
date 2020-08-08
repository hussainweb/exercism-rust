use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams: HashSet<&'a str> = HashSet::new();
    for candidate in possible_anagrams {
        if word.to_lowercase() != candidate.to_lowercase()
            && is_anagram(word.to_string(), candidate.to_string())
        {
            anagrams.insert(candidate);
        }
    }

    anagrams
}

fn is_anagram(word: String, candidate: String) -> bool {
    normalize_string(word) == normalize_string(candidate)
}

fn normalize_string(word: String) -> String {
    let mut chars: Vec<char> = word.to_lowercase().chars().collect();
    chars.sort();

    chars.into_iter().collect()
}
