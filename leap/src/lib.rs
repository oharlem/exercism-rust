pub fn is_leap_year(year: u64) -> bool {
    if year % 4 == 0 {
        return if year % 100 == 0 {
            year % 400 == 0
        } else {
            true
        };
    }
    false

    // Nice approach from https://exercism.io/tracks/rust/exercises/leap/solutions/fcc0b53e3ede45c5b233355ed46ade8e
    /*
    match (year % 4, year % 100, year % 400) {
        (0, 0, 0) => true,
        (0, 0, _) => false,
        (0, _, _) => true,
        (_, _, _) => false
    }
    */
}
