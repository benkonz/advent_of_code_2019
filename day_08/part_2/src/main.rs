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

    let mut image_buffer = vec![2; width * height];

    let num_layers = digits.len() / (width * height);

    for (i, pixel) in image_buffer.iter_mut().enumerate().take(width * height) {
        for j in 0..num_layers {
            let digit_index = j * width * height + i;
            match digits[digit_index] {
                0 | 1 => {
                    *pixel = digits[digit_index];
                    break;
                }
                _ => (),
            }
        }
    }

    for i in 0..height {
        for j in 0..width {
            let index = i * width + j;
            if image_buffer[index] == 1 {
                print!("{}", image_buffer[index]);
            } else {
                print!(" ");
            }
        }
        println!();
    }

    Ok(())
}
