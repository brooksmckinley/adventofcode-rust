use std::io;

fn get_line() -> String {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input).unwrap();
    input
}
fn main() {
    let mut line = get_line();
    let mut lines: Vec<String> = Vec::new();
    while &line != "" {
        lines.push(line);
        line = get_line();
    } 
    let mut a = "";
    let mut b = "";
    for i in 0..(lines.len() - 1) {
        let current = &lines[i];
        for j in (i + 1)..(lines.len()) {
            let mut score = 0;
            let test = &lines[j];
            for li in 0..current.len() {
                if test[li..li + 1] == current[li..li + 1] {
                    score = score + 1;
                }
            }
            if score == current.len() - 1 {
                a = current;
                b = test;
            }
        }
    }
    // not technically the solution, but the rest of the problem is faster done by eye than by computer
    // TODO: fix maybe?
    println!("{}{}", a, b);
}
