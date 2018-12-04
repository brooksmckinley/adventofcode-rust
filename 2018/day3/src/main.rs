use std::io;

#[derive(Copy, Debug, Clone)]
enum Square {
    Unclaimed,
    Claimed,
    DoubleClaimed,
}

fn get_line() -> String {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input).unwrap();
    input
}
fn main() {
    let mut line = get_line().trim().to_owned();
    let mut claims: Vec<String> = Vec::new();
    while line != "" {
        claims.push(line);
        line = get_line().trim().to_owned();
    }
    let mut fabric = [[Square::Unclaimed; 1000]; 1000];
    for request in claims.clone() {
        let parts: Vec<&str> = request.split(' ').collect();
        let start_x: usize = parts[2][0..parts[2].find(',').unwrap()].parse().unwrap();
        let start_y: usize = parts[2][parts[2].find(',').unwrap() + 1..parts[2].len() - 1].parse().unwrap();
        let claim_x: usize = parts[3][0..parts[3].find('x').unwrap()].parse().unwrap();
        let claim_y: usize = parts[3][parts[3].find('x').unwrap() + 1..].parse().unwrap();
        // Mark the parts of the fabric array
        for index in 0..claim_y {
            for index_x in 0..claim_x {
                let change = match fabric[index + start_y][index_x + start_x] {
                    Square::Unclaimed => Square::Claimed,
                    Square::Claimed => Square::DoubleClaimed,
                    Square::DoubleClaimed => Square::DoubleClaimed,
                };
                fabric[index + start_y][index_x + start_x] = change;
            }
        }
    }
    // Count the overlapping squares
    let mut double_claims = 0;
    for row in fabric.iter() {
        for square in row.iter() {
            if let Square::DoubleClaimed = square {
                double_claims = double_claims + 1;
            }
        }
    }
    println!("Part 1: {}", double_claims);
    // Go back through the array checking for a claim without overlapping values
    for request in claims {
        let parts: Vec<&str> = request.split(' ').collect();
        let request_name = &parts[0];
        let start_x: usize = parts[2][0..parts[2].find(',').unwrap()].parse().unwrap();
        let start_y: usize = parts[2][parts[2].find(',').unwrap() + 1..parts[2].len() - 1].parse().unwrap();
        let claim_x: usize = parts[3][0..parts[3].find('x').unwrap()].parse().unwrap();
        let claim_y: usize = parts[3][parts[3].find('x').unwrap() + 1..].parse().unwrap();
        let mut double_claimed = false;
        // Mark the parts of the fabric array
        for index in 0..claim_y {
            for index_x in 0..claim_x {
                match fabric[index + start_y][index_x + start_x] {
                    Square::DoubleClaimed => double_claimed = true,
                    _ => (),
                };
            }
        }
        if !double_claimed {
            println!("Part 2: {}", request_name);
        }
    }
}
