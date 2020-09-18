pub fn reply(message: &str) -> &str {
    /*
    conditions:
    1) question, contains letters, all capital, not empty
       "Calm down, I know what I'm doing!"
    2) question, any symbols, any caps, n/a
        "Sure."
    3) not a question, contains letters, all capital, not empty
       "Whoa, chill out!"
    4) everything else
       "Whatever."

        + need to trim whitespaces
     */
    let m = message.trim_end().replace('\r', "");
    let is_question = m.ends_with("?");
    let has_letters = m.chars().any(char::is_alphabetic);
    let all_capital = m == m.to_uppercase();
    let is_empty = m.is_empty();

    match (is_question, has_letters, all_capital, is_empty) {
        (true, true, true, false) => "Calm down, I know what I'm doing!",
        (true, _, _, _) => "Sure.",
        (false, true, true, false) => "Whoa, chill out!",
        (_, _, _, true) => "Fine. Be that way!",
        (_, _, _, _) => "Whatever.",
    }
}
