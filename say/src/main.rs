mod lib;

use std::process::Command;

fn main() {
    let n = lib::encode(123);

    let phrase_cmd = format!("say {}", n.to_string());

    Command::new("sh")
        .arg("-c")
        .arg(phrase_cmd)
        .output()
        .expect("failed to execute process");

    println!("{}", n)
}
