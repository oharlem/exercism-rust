pub fn encode(source: &str) -> String {
    // Example: "WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWB"  ->  "12WB12W3B24WB"
    // Algorithm:
    // - compare symbols with a previous one
    // - keep a counter of repetitions
    // - on symbols change,
    // -- store the resulting number and a symbol in result
    // -- if 1 repetition, skip 1 from result
    // -- reset counter
    let mut out = String::new();
    let mut prev: char = '0';
    let mut counter = 0;
    for c in source.chars() {
        if c != prev {
            if counter == 1 {
                out = format!("{}{}", out, prev);
            } else {
                out = format!("{}{}{}", out, counter, prev);
            }
            prev = c;
            counter = 1;
            continue;
        }

        counter += 1;
    }

    out
}

pub fn decode(source: &str) -> String {
    unimplemented!("Return the run-length decoding of {}.", source);
}
