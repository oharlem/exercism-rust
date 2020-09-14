use std::iter::once;

pub fn build_proverb(list: &[&str]) -> String {
    if list.len() == 0 {
        return String::new();
    }
    list.windows(2)
        .map(|pair| format!("For want of a {} the {} was lost.\n", pair[0], pair[1]))
        .chain(once(format!("And all for the want of a {}.", list[0])))
        .collect()

    // let mut res = String::new();
    // if list.is_empty() {
    //     return res;
    // }
    // for x in 0..list.len() - 1 {
    //     res.push_str(&format!("For want of a {} the {} was lost.\n", list[x], list[x + 1]));
    // }
    //
    // res.push_str(&format!("And all for the want of a {}.", list[0]));
    //
    // res
}
