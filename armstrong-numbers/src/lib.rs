pub fn is_armstrong_number(num: u32) -> bool {

    // accumulate digits here
    let mut digits: Vec<u32> = Vec::new();

    // extract digits
    let mut n = num;
    loop {
        let d = n % 10;

        if d != 0 {
            digits.push(d);
            n -= d;
        }
        n /= 10;

        if n < 1 {
            break;
        }
    }

    let len = digits.len() as u32;

    let sum: u32 = digits
        .into_iter()
        .map(|x| x.pow(len))
        .sum();

    sum == num
}