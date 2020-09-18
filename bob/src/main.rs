use bob::reply;

fn main() {
    let m = reply("\r\r ");

    println!("{}", m);
}