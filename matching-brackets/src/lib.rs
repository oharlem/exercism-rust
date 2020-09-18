use unicode_segmentation::UnicodeSegmentation;

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<&str> = Vec::new();

    /*
    Algorithm:
    - if an opening bracket: add to the stack
    - if a closing bracket, check if matches
    -- if no match, fail
    -- if match, pop existing bracket
     */

    let open: Vec<&str> = vec!["(", "{", "["];
    let close: Vec<&str> = vec![")", "}", "]"];
    let s: Vec<&str> = string.graphemes(true).collect();

    for c in s {
        // ignore non-brackets
        if !open.contains(&c) && !close.contains(&c) {
            continue;
        }

        if stack.is_empty() {
            if close.contains(&c) {
                return false;
            }
            stack.push(c);
            continue;
        }

        if open.contains(&c) {
            // opening bracket
            stack.push(c);
        } else {
            // closing bracket
            // compare with the last bracket,
            // if no match - fail, if match - remove
            let top = stack.pop().unwrap();
            if top != is_pair(c) {
                return false;
            }
        }
    }

    stack.is_empty()
}

fn is_pair(c: &str) -> &str {
    if c == "]" {
        return "[";
    }
    if c == "}" {
        return "{";
    }
    if c == ")" {
        return "(";
    }

    ""
}
