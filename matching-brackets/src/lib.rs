use unicode_segmentation::UnicodeSegmentation;

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

    let mut stack: Vec<&str> = Vec::new();

    // convert to graphemes and leave only characters relevant for the function
    let s: Vec<&str> = string
        .graphemes(true)
        .filter(|c| "[]{}()".contains(c))
        .collect();

    for c in s {
        if stack.is_empty() {
            if ")}]".contains(&c) {
                return false;
            }
            stack.push(c);
            continue;
        }

        // opening bracket
        if "({[".contains(&c) {
            stack.push(c);
            continue;
        }

        // closing bracket
        // compare with the last bracket,
        // if no match - fail, if match - remove
        if !is_pair(stack.pop().unwrap(), c) {
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
