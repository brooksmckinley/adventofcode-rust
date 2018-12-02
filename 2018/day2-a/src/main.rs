use std::io;

fn get_line() -> String {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input).unwrap();
    input
}
fn main() {
    let mut line = get_line();
    let mut twos = 0;
    let mut threes = 0;
    while &line != "" {
        let mut letters = [0; 26];
        for c in line.chars() {
            if c as usize >= 97 {
                letters[c as usize - 97] = letters[c as usize - 97] + 1;
            }
        }
        let mut two = false;
        let mut three = false;
        for l in &letters {
            if l == &2 {
                two = true;
            }
            else if l == &3 {
                three = true;
            }
        }
        if two {
            twos = twos + 1;
        }
        if three {
            threes = threes + 1;
        }
        line = get_line();
    }
    println!("{}", twos * threes);
}
