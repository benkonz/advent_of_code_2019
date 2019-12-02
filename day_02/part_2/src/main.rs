use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let program: Vec<i32> = buffer
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    for noun in 0..100 {
        for verb in 0..100 {
            let mut modified_program = program.to_vec();
            modified_program[1] = noun;
            modified_program[2] = verb;
            execute_program(&mut modified_program);

            if modified_program[0] == 19_690_720 {
                println!("noun: {}, verb {}", noun, verb);
                println!("answer: {}", 100 * noun + verb);
                break;
            }
        }
    }

    Ok(())
}

fn execute_program(program: &mut [i32]) {
    let mut pc = 0;
    while pc < program.len() {
        let opcode = program[pc];
        pc += 1;
        match opcode {
            1 => {
                let (pos1, pos2, pos3) = (program[pc], program[pc + 1], program[pc + 2]);
                pc += 3;
                program[pos3 as usize] = program[pos1 as usize] + program[pos2 as usize]
            }
            2 => {
                let (pos1, pos2, pos3) = (program[pc], program[pc + 1], program[pc + 2]);
                pc += 3;
                program[pos3 as usize] = program[pos1 as usize] * program[pos2 as usize]
            }
            99 => break,
            _ => panic!("unknown opcode: {}", opcode),
        }
    }
}
