use std::collections::VecDeque;
use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let mut program: Vec<i64> = buffer
        .trim()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    program.extend_from_slice(&[0; 1000]);

    let mut stdin = VecDeque::new();
    stdin.push_back("2".to_string());

    execute_program(&mut program, &mut stdin);

    Ok(())
}

fn execute_program(program: &mut [i64], stdin: &mut VecDeque<String>) {
    let mut pc = 0;
    let mut relative_base = 0;
    while pc < program.len() {
        let opcode_number = program[pc];
        let opcode = get_opcode(opcode_number);
        pc += 1;
        match opcode {
            1 => {
                let (mode1, mode2, mode3) = (
                    get_digit(opcode_number, 2),
                    get_digit(opcode_number, 3),
                    get_digit(opcode_number, 4),
                );

                let arg1 = get_arg(program, pc, mode1, relative_base);
                pc += 1;
                let arg2 = get_arg(program, pc, mode2, relative_base);
                pc += 1;
                let arg3 = get_arg(program, pc, 1, relative_base);
                pc += 1;
                let value = arg1 + arg2;

                store_value(program, relative_base, mode3, arg3, value);
            }
            2 => {
                let (mode1, mode2, mode3) = (
                    get_digit(opcode_number, 2),
                    get_digit(opcode_number, 3),
                    get_digit(opcode_number, 4),
                );

                let arg1 = get_arg(program, pc, mode1, relative_base);
                pc += 1;
                let arg2 = get_arg(program, pc, mode2, relative_base);
                pc += 1;
                let arg3 = get_arg(program, pc, 1, relative_base);
                pc += 1;
                let value = arg1 * arg2;

                store_value(program, relative_base, mode3, arg3, value);
            }
            3 => {
                let mode = get_digit(opcode_number, 2);
                let buffer = stdin.pop_front().unwrap();
                let arg = get_arg(program, pc, 1, relative_base);
                pc += 1;

                let value = buffer.parse::<i64>().unwrap();
                store_value(program, relative_base, mode, arg, value);
            }
            4 => {
                let mode = get_digit(opcode_number, 2);
                let arg1 = get_arg(program, pc, mode, relative_base);
                pc += 1;
                println!("{}", arg1);
            }
            5 => {
                let (mode1, mode2) = (get_digit(opcode_number, 2), get_digit(opcode_number, 3));

                let arg1 = get_arg(program, pc, mode1, relative_base);
                pc += 1;
                let arg2 = get_arg(program, pc, mode2, relative_base);
                pc += 1;
                if arg1 != 0 {
                    pc = arg2 as usize;
                }
            }
            6 => {
                let (mode1, mode2) = (get_digit(opcode_number, 2), get_digit(opcode_number, 3));

                let arg1 = get_arg(program, pc, mode1, relative_base);
                pc += 1;
                let arg2 = get_arg(program, pc, mode2, relative_base);
                pc += 1;
                if arg1 == 0 {
                    pc = arg2 as usize;
                }
            }
            7 => {
                let (mode1, mode2, mode3) = (
                    get_digit(opcode_number, 2),
                    get_digit(opcode_number, 3),
                    get_digit(opcode_number, 4),
                );

                let arg1 = get_arg(program, pc, mode1, relative_base);
                pc += 1;
                let arg2 = get_arg(program, pc, mode2, relative_base);
                pc += 1;
                let arg3 = get_arg(program, pc, 1, relative_base);
                pc += 1;
                let value = if arg1 < arg2 { 1 } else { 0 };

                store_value(program, relative_base, mode3, arg3, value);
            }
            8 => {
                let (mode1, mode2, mode3) = (
                    get_digit(opcode_number, 2),
                    get_digit(opcode_number, 3),
                    get_digit(opcode_number, 4),
                );

                let arg1 = get_arg(program, pc, mode1, relative_base);
                pc += 1;
                let arg2 = get_arg(program, pc, mode2, relative_base);
                pc += 1;
                let arg3 = get_arg(program, pc, 1, relative_base);
                pc += 1;
                let value = if arg1 == arg2 { 1 } else { 0 };

                store_value(program, relative_base, mode3, arg3, value);
            }
            9 => {
                let mode = get_digit(opcode_number, 2);
                let arg = get_arg(program, pc, mode, relative_base);
                pc += 1;
                relative_base += arg;
            }
            99 => break,
            _ => panic!("unknown opcode: {}", opcode),
        }
    }
}

fn get_opcode(opcode_number: i64) -> i64 {
    get_digit(opcode_number, 0) + (10 * get_digit(opcode_number, 1))
}

fn get_digit(number: i64, position: u32) -> i64 {
    number / i64::pow(10, position) % 10
}

fn get_arg(program: &[i64], pc: usize, mode: i64, relative_base: i64) -> i64 {
    match mode {
        0 => program[program[pc] as usize],
        1 => program[pc],
        2 => program[(program[pc] + relative_base) as usize],
        _ => panic!("unknown mode!"),
    }
}

fn store_value(program: &mut [i64], relative_base: i64, mode: i64, arg: i64, value: i64) {
    let index = if mode == 2 {
        (arg + relative_base) as usize
    } else {
        arg as usize
    };
    program[index] = value;
}
