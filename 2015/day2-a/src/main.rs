use std::io;

fn get_line() -> String {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input).unwrap();
    input
}
fn main() {
    let mut total = 0;
    // Loop until the line doesn't include enough numbers for the calculation
    loop {
        let line = get_line();
        let line = line.trim();
        // Collect all of the numbers into a Vec
        let nums: Vec<&str> = line.split("x").collect();
        // If the Vec isn't long enough, break the loop
        if nums.len() < 3 {
            break;
        }
        let length: i32 = nums[0].parse().unwrap();
        let width: i32 = nums[1].parse().unwrap();
        let height: i32 = nums[2].parse().unwrap();
        let surface_area = (2 * length * width) + (2 * width * height) + (2 * height * length);
        total += surface_area;

        // Calculate slack
        let mut sides = vec![length, width, height];
        sides.sort();
        let slack = sides[0] * sides[1];
        total += slack;
    }
    println!("Total surface area: {}", total);
}
