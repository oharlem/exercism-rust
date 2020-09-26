use std::ops::Rem;

pub fn encode(n: u64) -> String {
    if n == 0 {
        return String::from("zero");
    }

    let mut out = String::new();
    let mut chunks = Vec::new();
    let mut n2 = n.clone();

    // split the number into chunks by thousands
    while n2 > 0 {
        chunks.push(n2.rem(1000));
        n2 /= 1000;
    }

    // convert the chunks to a number string
    // after every number from the end, add next word/thousand part descriptor
    let ns = vec!["", "thousand", "million", "billion", "trillion", "quadrillion", "quintillion"];
    let chunk_pairs: Vec<_> = chunks.iter().enumerate().collect();
    for chunk in chunk_pairs {
        // process < 1000 differently
        if chunk.0 == 0 {
            out = format!("{}", conv(*chunk.1));
            out = format!("{}", out.trim());
            continue;
        }
        // skip 000
        if *chunk.1 == 0 {
            continue;
        }
        out = format!("{} {} {}", conv(*chunk.1), ns[chunk.0], out);
        out = format!("{}", out.trim());
    }

    out
}

// convert numbers up to 999 to string
pub fn conv(n: u64) -> String {
    let mut out = String::new();

    if n > 99 {
        match n.to_string().chars().nth(0).unwrap() {
            '1' => out = String::from("one"),
            '2' => out = String::from("two"),
            '3' => out = String::from("three"),
            '4' => out = String::from("four"),
            '5' => out = String::from("five"),
            '6' => out = String::from("six"),
            '7' => out = String::from("seven"),
            '8' => out = String::from("eight"),
            _ => out = String::from("nine"),
        }

        out = format!("{} hundred", out);
    }

    // remainder
    let n2 = n % 100;

    if n2 <= 19 {
        match n2 {
            1 => out = format!("{} one", out),
            2 => out = format!("{} two", out),
            3 => out = format!("{} three", out),
            4 => out = format!("{} four", out),
            5 => out = format!("{} five", out),
            6 => out = format!("{} six", out),
            7 => out = format!("{} seven", out),
            8 => out = format!("{} eight", out),
            9 => out = format!("{} nine", out),
            10 => out = format!("{} ten", out),
            11 => out = format!("{} eleven", out),
            12 => out = format!("{} twelve", out),
            13 => out = format!("{} thirteen", out),
            14 => out = format!("{} fourteen", out),
            15 => out = format!("{} fifteen", out),
            16 => out = format!("{} sixteen", out),
            17 => out = format!("{} seventeen", out),
            18 => out = format!("{} eighteen", out),
            19 => out = format!("{} nineteen", out),
            _ => out = format!("{}", out),
        }

        return out;
    }

    let nstr = (n % 100).to_string();
    let mut cpos = 0;

    if n > 19 {
        match nstr.chars().nth(0).unwrap() {
            '2' => out = format!("{} twenty", out),
            '3' => out = format!("{} thirty", out),
            '4' => out = format!("{} forty", out),
            '5' => out = format!("{} fifty", out),
            '6' => out = format!("{} sixty", out),
            '7' => out = format!("{} seventy", out),
            '8' => out = format!("{} eighty", out),
            _ => out = format!("{} ninety", out),
        }

        cpos = 1;
    }

    match nstr.chars().nth(cpos).unwrap() {
        '1' => out = format!("{}-one", out),
        '2' => out = format!("{}-two", out),
        '3' => out = format!("{}-three", out),
        '4' => out = format!("{}-four", out),
        '5' => out = format!("{}-five", out),
        '6' => out = format!("{}-six", out),
        '7' => out = format!("{}-seven", out),
        '8' => out = format!("{}-eight", out),
        '9' => out = format!("{}-nine", out),
        _ => out = format!("{}", out),
    }

    out
}