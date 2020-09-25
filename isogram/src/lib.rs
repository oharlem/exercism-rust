pub fn check(candidate: &str) -> bool {
    // none of the symbols should repeat except spaces and hyphens
    let s = candidate
        .chars()
        .filter(|x| *x != ' ' && *x != '-')
        .map(|x| x.to_ascii_lowercase());

    let mut v = Vec::new();
    for c in s {
        if v.contains(&c) {
            return false;
        }
        v.push(c);
    }

    true
}

// Functional implementation
use std::collections::HashSet;

pub fn check_functional(candidate: &str) -> bool {
    let mut set = HashSet::new();

    candidate
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .all(|c| set.insert(c))
}
