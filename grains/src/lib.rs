pub fn square(s: u32) -> u64 {
    match s {
        1 => 1 as u64,
        2..=64 => {
            let sum: u64 = 2;
            sum.pow(s - 1)
        }
        _ => panic!("Square must be between 1 and 64"),
    }
}

pub fn total() -> u64 {
    (square(64) - 1) * 2 + 1
}
