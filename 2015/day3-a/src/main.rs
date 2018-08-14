use std::io;

#[derive(PartialEq, Clone, Debug)]
struct House {
    x: i32,
    y: i32,
}

fn get_line() -> String {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input).unwrap();
    input
}
fn main() {
    let input = get_line();
    let mut houses: Vec<House> = Vec::new();
    let mut unique_houses = 1;
    let mut current_house = House { x: 0, y: 0 };
    // Santa delivers a present to himself
    houses.push(current_house.clone());
    for letter in input.as_str().chars() {
        // Follow drunk elf's instructions
        match letter {
            '^' => current_house.y += 1,
            'v' => current_house.y -= 1,
            '>' => current_house.x += 1,
            '<' => current_house.x -= 1,
            _ => (),
        }
        if !houses.contains(&current_house) {
            unique_houses += 1;
            houses.push(current_house.clone());
        }
    }
    println!("Unique houses: {}", unique_houses);
}
