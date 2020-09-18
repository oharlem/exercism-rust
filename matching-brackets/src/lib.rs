pub fn brackets_are_balanced(string: &str) -> bool {
    /*
    Algorithm:
    - if an opening bracket: add to the stack
    - if a closing bracket, check if matches
    -- if no match, fail
    -- if match, pop existing bracket
     */

    if string.is_empty() {
        return true;
    }

    let mut stack: Vec<char> = Vec::new();

    let s: Vec<char> = string
        .chars()
        .filter(|c| "[]{}()".contains(&c.to_string()))
        .collect();

    for c in s {
        let sc = &c.to_string();

        if stack.is_empty() {
            // first element cannot be a closing bracket
            if ")}]".contains(sc) {
                return false;
            }
            stack.push(c);
            continue;
        }

        // opening bracket
        if "({[".contains(sc) {
            stack.push(c);
            continue;
        }

        // closing bracket
        // compare with the last bracket,
        // if no match - fail, if match - remove
        if !is_pair(&stack.pop().unwrap().to_string(), sc) {
            return false;
        }
    }

    // if all pairs match, stack should be cleaned by the end
    stack.is_empty()
}

fn is_pair(top: &str, cur: &str) -> bool {
    match (top, cur) {
        ("[", "]") | ("(", ")") | ("{", "}") => true,
        _ => false,
    }
}
