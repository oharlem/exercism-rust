fn is_prime(n: u32) -> bool {

    // If i can divide n on any number from 2 to n-1,
    // i.e. any number except of 1 and itself,
    // then it is not prime.
    for x in 2..n {
        if n % x == 0 {
            return false;
        }
    }

    true
}

pub fn nth(n: u32) -> u32 {
    // algorithm:
    // run a loop incrementing current numbers by 1
    // checking every number to be prime
    // incrementing a counter if prime
    // until counter == n

    // looks like authors guarantee that nth value is withing u32
    let mut counter: u32 = 0;
    let mut i: u32 = 2;

    loop {
        if is_prime(i) {
            if counter == n {
                return i;
            }
            counter += 1;
        }
        i += 1;
    }

}
