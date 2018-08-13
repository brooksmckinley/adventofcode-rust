use std::io;

fn get_line() -> String {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input);
    input
}
fn main() {
    println!("Hello, world!");
}
