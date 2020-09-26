use std::ops::Rem;

pub fn encode(n: u64) -> String {
    // if n > 1_000_000_000 {
    //     return String::from("out of range");
    // }
    //
    // if n == 0 {
    //     return String::from("zero");
    // }

    let mut out = String::new();
    let mut chunks = Vec::new();
    let mut n2 = n.clone();
    n2 = 1234567890;

    // split the number into chunks by thousands
    while n2 > 0 {
        chunks.push(n2.rem(1000));
        n2 /= 1000;
    }
    // chunks.reverse();

    // convert the chunks to a number string
    // after every number from the end, add next word/thousand part descriptor
    let ns = vec!["", "thousand", "million", "billion"];
    let chunk_pairs: Vec<_> = chunks.iter().enumerate().collect();
    for chunk in chunk_pairs {
        println!("CURRENT CHUNK: {:?}", chunk);
        if chunk.0 == 0 {
            out = format!("{}", chunk.1);
            continue;
        }
        out = format!("{} {} {}", chunk.1, ns[chunk.0], out);
    }
    out.trim();

    println!("N: {}", n2);
    println!("chunks: {:#?}", chunks);
    println!("OUT: {:?}", out);

    if n >= 10 && n <= 19 {
        match n {
            10 => out = String::from("ten"),
            11 => out = String::from("eleven"),
            12 => out = String::from("twelve"),
            13 => out = String::from("thirteen"),
            14 => out = String::from("fourteen"),
            15 => out = String::from("fifteen"),
            16 => out = String::from("sixteen"),
            17 => out = String::from("seventeen"),
            18 => out = String::from("eighteen"),
            _ => out = String::from("nineteen"),
        }

        return out;
    }

    let nstr = n.to_string();

    let mut c2pos = 0;

    if nstr.len() == 2 {
        match nstr.chars().nth(0).unwrap() {
            '2' => out = String::from("twenty"),
            '3' => out = String::from("thirty"),
            '4' => out = String::from("fourty"),
            '5' => out = String::from("fifty"),
            '6' => out = String::from("sixty"),
            '7' => out = String::from("seventy"),
            '8' => out = String::from("eighty"),
            _ => out = String::from("ninety"),
        }

        c2pos = 1;
    }

    match nstr.chars().nth(c2pos).unwrap() {
        '1' => out = format!("{} {}", out, "one"),
        '2' => out = format!("{} {}", out, "two"),
        '3' => out = format!("{} {}", out, "three"),
        '4' => out = format!("{} {}", out, "four"),
        '5' => out = format!("{} {}", out, "five"),
        '6' => out = format!("{} {}", out, "six"),
        '7' => out = format!("{} {}", out, "seven"),
        '8' => out = format!("{} {}", out, "eight"),
        _ => out = format!("{} {}", out, "nine"),
    }


    out
}
