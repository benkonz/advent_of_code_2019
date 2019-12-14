use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let mut program: Vec<i32> = buffer
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    execute_program(&mut program)?;

    Ok(())
}

fn execute_program(program: &mut [i32]) -> io::Result<()> {
    let mut pc = 0;
    while pc < program.len() {
        let opcode_number = program[pc];
        let opcode = get_opcode(opcode_number);
        pc += 1;
        match opcode {
            1 => {
                let (mode1, mode2) = (get_digit(opcode_number, 2), get_digit(opcode_number, 3));

                let arg1 = get_arg(program, pc, mode1);
                pc += 1;
                let arg2 = get_arg(program, pc, mode2);
                pc += 1;
                let arg3 = get_arg(program, pc, 1);
                pc += 1;
                program[arg3 as usize] = arg1 + arg2;
            }
            2 => {
                let (mode1, mode2) = (get_digit(opcode_number, 2), get_digit(opcode_number, 3));

                let arg1 = get_arg(program, pc, mode1);
                pc += 1;
                let arg2 = get_arg(program, pc, mode2);
                pc += 1;
                let arg3 = get_arg(program, pc, 1);
                pc += 1;
                program[arg3 as usize] = arg1 * arg2;
            }
            3 => {
                let mut buffer = String::new();
                io::stdin().read_line(&mut buffer)?;
                let arg1 = get_arg(program, pc, 1);
                pc += 1;
                program[arg1 as usize] = buffer.parse::<i32>().unwrap();
            }
            4 => {
                let mode = get_digit(opcode_number, 2);
                let arg1 = get_arg(program, pc, mode);
                pc += 1;
                println!("{}", arg1);
            }
            5 => {
                let (mode1, mode2) = (get_digit(opcode_number, 2), get_digit(opcode_number, 3));

                let arg1 = get_arg(program, pc, mode1);
                pc += 1;
                let arg2 = get_arg(program, pc, mode2);
                pc += 1;
                if arg1 != 0 {
                    pc = arg2 as usize;
                }
            }
            6 => {
                let (mode1, mode2) = (get_digit(opcode_number, 2), get_digit(opcode_number, 3));

                let arg1 = get_arg(program, pc, mode1);
                pc += 1;
                let arg2 = get_arg(program, pc, mode2);
                pc += 1;
                if arg1 == 0 {
                    pc = arg2 as usize;
                }
            }
            7 => {
                let (mode1, mode2) = (get_digit(opcode_number, 2), get_digit(opcode_number, 3));

                let arg1 = get_arg(program, pc, mode1);
                pc += 1;
                let arg2 = get_arg(program, pc, mode2);
                pc += 1;
                let arg3 = get_arg(program, pc, 1);
                pc += 1;
                let value = if arg1 < arg2 { 1 } else { 0 };
                program[arg3 as usize] = value;
            }
            8 => {
                let (mode1, mode2) = (get_digit(opcode_number, 2), get_digit(opcode_number, 3));

                let arg1 = get_arg(program, pc, mode1);
                pc += 1;
                let arg2 = get_arg(program, pc, mode2);
                pc += 1;
                let arg3 = get_arg(program, pc, 1);
                pc += 1;
                let value = if arg1 == arg2 { 1 } else { 0 };
                program[arg3 as usize] = value;
            }
            99 => break,
            _ => panic!("unknown opcode: {}", opcode),
        }
    }

    Ok(())
}

fn get_opcode(opcode_number: i32) -> i32 {
    get_digit(opcode_number, 0) + (10 * get_digit(opcode_number, 1))
}

fn get_digit(number: i32, position: u32) -> i32 {
    number / i32::pow(10, position) % 10
}

fn get_arg(program: &[i32], pc: usize, mode: i32) -> i32 {
    match mode {
        0 => program[program[pc] as usize],
        1 => program[pc],
        _ => panic!("unknown mode!"),
    }
}
