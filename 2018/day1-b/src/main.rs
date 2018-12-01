use std::io;
use std::collections::HashSet;

fn get_line() -> String {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input).unwrap();
    input
}
fn main() {
    let mut freq: i32 = 0;
    let mut input: Vec<i32> = Vec::new();
    let mut line = get_line();
    while line.as_str() != "" {
        input.push(line.trim().parse::<i32>().unwrap());
        line = get_line();
    }
    let mut freqs: HashSet<i32> = HashSet::new();
    freqs.insert(0);
    let mut res = 0;
    let mut c = true;
    let mut reps = 0;
    while c {
        println!("reps: {}", reps);
        for i in &input {
            freq = freq + i;
            if freqs.contains(&freq) {
                res = freq;
                c = false;
                break;
            }
            else {
                freqs.insert(freq);
            }
        }
        reps = reps + 1;
    }
    println!("res: {}", res);
}
