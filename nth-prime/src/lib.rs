fn is_prime(n: u32) -> bool {
    // can run division on the first half only, because
    // divisors form pairs and does not make sense to divide by a 2nd part of a pair
    // if it is already divisible by 1st part

    // my original approach
    // for x in 2..n/2 + 1 {
    //     if n % x == 0 {
    //         return false;
    //     }
    // }
    //
    // true

    // a more Rusty approach
    // ..playing with Rust so decided to leave one while time shows it to be slower
    !(2..n / 2 + 1).any(|x| n % x == 0)
}

pub fn nth(n: u32) -> u32 {
    // algorithm:
    // run a loop incrementing current numbers by 1
    // checking every number to be prime
    // incrementing a counter if prime
    // until counter == n

    (2..)
        .filter(|x| is_prime(*x))
        .nth(n as usize)
        .unwrap()
}
