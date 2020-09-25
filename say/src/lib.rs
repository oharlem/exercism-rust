pub fn encode(n: u64) -> String {
    let mut out = String::new();

    if n > 99 {
        return String::from("out of range");
    }

    if n == 0 {
        return String::from("zero");
    }

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
