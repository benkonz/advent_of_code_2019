use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let digits: Vec<u32> = buffer
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let width = 25;
    let height = 6;

    let num_layers = digits.len() / (width * height);

    let mut fewest_zeroes = std::u32::MAX;
    let mut answer = -1;
    for i in 0..num_layers {
        let mut current_zeroes = 0;
        let mut current_ones = 0;
        let mut current_twos = 0;
        for j in 0..width * height {
            let index = (i * (width * height) + j) as usize;
            match digits[index] {
                0 => current_zeroes += 1,
                1 => current_ones += 1,
                2 => current_twos += 1,
                _ => (),
            };
        }
        if current_zeroes < fewest_zeroes {
            answer = current_ones * current_twos;
            fewest_zeroes = current_zeroes;
        }
    }
    println!("{}", answer);

    Ok(())
}
