use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let parsed: Vec<i32> = buffer
        .trim()
        .split('-')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let (low, high) = (parsed[0], parsed[1]);

    let mut num_passwords = 0;
    for candidate in low..=high {
        if has_double_adjacent_digits(candidate) && has_ascending_digits(candidate) {
            num_passwords += 1;
        }
    }
    println!("{}", num_passwords);

    Ok(())
}

fn has_double_adjacent_digits(number: i32) -> bool {
    let number_str = format!("{}", number);
    for i in 1..number_str.len() {
        if number_str.chars().nth(i - 1) == number_str.chars().nth(i) {
            return true;
        }
    }
    false
}

fn has_ascending_digits(number: i32) -> bool {
    let number_str = format!("{}", number);
    for i in 1..number_str.len() {
        if number_str.chars().nth(i - 1) > number_str.chars().nth(i) {
            return false;
        }
    }
    true
}
