use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let total_fuel: i32 = buffer
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .map(compute_fuel)
        .sum();
    println!("{}", total_fuel);
    Ok(())
}

fn compute_fuel(mass: i32) -> i32 {
    mass / 3 - 2
}
