use std::io;

fn get_line() -> String {
    let mut out = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut out).unwrap();
    out
}

fn main() {
    let input = get_line();
    let mut floor: i32 = 0;
    let mut position = 0;
    for letter in input.as_str().chars() {
        position += 1;
        if letter == '(' {
            floor = floor + 1;
        }
        else if letter == ')' {
            floor = floor - 1;
        }
        // Check if Santa has reached the basement yet
        if floor == -1 {
            break;
        }
    }
    println!("Position where Santa first entered the basement: {}", position);
}
