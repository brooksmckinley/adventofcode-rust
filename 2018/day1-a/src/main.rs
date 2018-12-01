use std::io;

fn get_line() -> String {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input).unwrap();
    input
}
fn main() {
    let mut freq = 0;
    let mut line = get_line();
    while line.as_str() != "" {
        freq = freq + line.trim().parse::<i32>().unwrap();
        println!("freq: {}", freq);
        line = get_line();
    }
}
