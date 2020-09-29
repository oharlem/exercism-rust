use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    /*
    Anagram: same number of unique letters with same quantities

    Algorithm:
    -- create a hashmap of letters-to-quantities for the input word
    -- create a hashmap of letters-to-quantities for each word
    -- if they match, add to the output
     */

    let hm_word = word_to_hashmap(&word);
    let mut out = HashSet::new();
    for a in possible_anagrams {
        if word.to_lowercase() != *a.to_lowercase()
            && hm_word == word_to_hashmap(&a) {
            out.insert(*a);
        }
    }

    out
}

fn word_to_hashmap(word: &str) -> HashMap<char, usize> {
    let w = word.to_lowercase();
    let mut hm = HashMap::new();

    for c in w.chars() {
        *hm.entry(c).or_insert(0) += 1;
    }

    hm
}
