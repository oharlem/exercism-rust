pub fn verse(n: u32) -> String {
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        n => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle{} of beer on the wall.\n", n, n, n - 1, if n - 1 > 1 { "s" } else { "" }),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(verse)
        .collect::<Vec<String>>()
        .join("\n")
}

// My previous solution:
// pub fn verse(n: u32) -> String {
//     if n > 2 {
//         return format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n - 1);
//     } else if n == 2 {
//         return format!("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n");
//     } else if n == 1 {
//         return format!("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n");
//     }
//
//     return format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
// }
//
// pub fn sing(start: u32, end: u32) -> String {
//     let mut res = String::new();
//     let mut x = start + 1;
//     while x > end {
//         x -= 1;
//
//         res.push_str(&format!("{}", verse(x)));
//         if x > end {
//             res.push_str("\n");
//         }
//     }
//     res.to_string()
// }