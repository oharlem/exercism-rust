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
