use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    /*
    Anagram: same number of unique letters with same quantities

    Algorithm:
    -- create a hashmap of letters-to-quantities for the input word
    -- create a hashmap of letters-to-quantities for each word
    -- if they match, add to the output
     */

    let wl = word.to_lowercase();
    let hm_word = word_to_hashmap(&wl);
    let mut out = HashSet::new();
    for a in possible_anagrams {
        let al = a.to_lowercase();
        if wl != al
            && hm_word == word_to_hashmap(&al)
        {
            out.insert(*a);
        }
    }

    out
}

fn word_to_hashmap(word: &str) -> HashMap<char, usize> {
    let mut hm = HashMap::new();

    for c in word.chars() {
        *hm.entry(c).or_insert(0) += 1;
    }

    hm
}
