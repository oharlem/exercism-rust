const PLACE_VALUES: [&str; 7] = [
    "",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
];

pub fn encode(n: u64) -> String {
    if n == 0 {
        return String::from("zero");
    }

    let mut chunks: Vec<(usize, u64)> = Vec::new();
    let mut n2 = n.clone();

    // split the number into chunks by thousands
    let mut c = 0;
    while n2 > 0 {
        // could use functional approach, but imperative is faster
        let tmp = n2 % 1000;
        // skip 000
        if tmp != 0 {
            chunks.push((c, tmp));
        }
        n2 /= 1000;
        c += 1;
    }

    let mut out: Vec<String> = Vec::new();
    for chunk in chunks {
        // process < 1000 differently
        if chunk.0 == 0 {
            out.push(conv(chunk.1));
        } else {
            out.push(format!("{} {}", conv(chunk.1), PLACE_VALUES[chunk.0]));
        }
    }

    out.reverse();
    out.join(" ")
}

// convert numbers up to 999 to string
pub fn conv(n: u64) -> String {
    let mut out: String = String::new();

    if n > 99 {
        let tmp_100 = match n / 100 {
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            5 => "five",
            6 => "six",
            7 => "seven",
            8 => "eight",
            _ => "nine",
        };

        out = format!("{} hundred", tmp_100);
    }

    // tens
    let n100 = n % 100;
    if n100 <= 19 {
        let n100_tmp = match n100 {
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            5 => "five",
            6 => "six",
            7 => "seven",
            8 => "eight",
            9 => "nine",
            10 => "ten",
            11 => "eleven",
            12 => "twelve",
            13 => "thirteen",
            14 => "fourteen",
            15 => "fifteen",
            16 => "sixteen",
            17 => "seventeen",
            18 => "eighteen",
            19 => "nineteen",
            _ => return out,
        };

        if out != "" {
            return format!("{} {}", out, n100_tmp);
        }

        return format!("{}", n100_tmp);
    }

    let n0 = match n100 / 10 {
        2 => "twenty",
        3 => "thirty",
        4 => "forty",
        5 => "fifty",
        6 => "sixty",
        7 => "seventy",
        8 => "eighty",
        _ => "ninety",
    };

    if out != "" {
        out = format!("{} {}", out, n0);
    } else {
        out = format!("{}", n0);
    }

    let n1 = match n % 10 {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => return out,
    };

    format!("{}-{}", out, n1)
}
