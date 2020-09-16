pub fn factors(n: u64) -> Vec<u64> {
    /*
   goal: find list of smallest factors
   cannot use 1 so we start with 2
   divide by 2
    divide quotient by 2 etc
   if cannot divide by current factor, increment by 1
     */
    let mut f = 2;
    let mut num = n;
    let mut pf = Vec::new();
    while num != 1 {
        if num % f == 0 {
            // add f to a list of factors
            pf.push(f);
            // move on to a quotient which is num / f
            num /= f;
        } else {
            // increment a factor to find an appropriate one
            f += 1;
        }
    }

    pf
}
