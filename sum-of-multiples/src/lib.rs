pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|x|

            // because we check if this number is a multiple
            // of _any_ of the factors, we do not add it several times
            // if it is a multiple of several factors,
            // ex. we do not add 15 twice if our factors are 3 and 5
            factors
                .iter()
                .any(|f|
                    *f != 0 && x % f == 0
                )
        )
        .sum()
}
