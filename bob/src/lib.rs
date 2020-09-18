pub fn reply(message: &str) -> &str {
    let m = message.trim_end().replace('\r', "");
    let all_cap = m == m.to_uppercase();
    let any_alpha = m.chars().any(char::is_alphabetic);
    let is_question = m.ends_with("?");

    if is_question {
        if any_alpha && all_cap {
            // yelling a question
            return "Calm down, I know what I'm doing!";
        }

        // all other questions, inc. input w/o letters
        return "Sure.";
    }

    // yelling, not a question
    if any_alpha && all_cap && !is_question {
        return "Whoa, chill out!";
    }

    // addressing w/o saying anything (empty string)
    if m.is_empty() {
        return "Fine. Be that way!";
    }

    "Whatever."
}
