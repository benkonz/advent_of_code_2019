use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let mut program: Vec<i32> = buffer
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    execute_program(&mut program[..]);
    println!("{}", program[0]);
    Ok(())
}

fn execute_program(program: &mut [i32]) {
    let mut pc = 0;
    while pc < program.len() {
        let opcode = program[pc];
        match opcode {
            1 => {
                let (pos1, pos2, pos3) = (program[pc + 1], program[pc + 2], program[pc + 3]);
                program[pos3 as usize] = program[pos1 as usize] + program[pos2 as usize]
            }
            2 => {
                let (pos1, pos2, pos3) = (program[pc + 1], program[pc + 2], program[pc + 3]);
                program[pos3 as usize] = program[pos1 as usize] * program[pos2 as usize]
            }
            99 => break,
            _ => panic!("unknown opcode: {}", opcode),
        }
        pc += 4;
    }
}
