use std::cmp;
use std::collections::VecDeque;
use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let program: Vec<i32> = buffer
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut vec = vec![0, 1, 2, 3, 4];
    let mut permutations = Vec::new();
    generate_permutations(&mut vec, 5, &mut permutations);

    let mut highest_signal = 0;
    for permutation in permutations {
        let (i, j, k, l, m) = (
            permutation[0],
            permutation[1],
            permutation[2],
            permutation[3],
            permutation[4],
        );
        let mut stdin_a = VecDeque::new();
        stdin_a.push_back(i);
        stdin_a.push_back(0);
        let mut stdout_a = execute_program(&mut program.to_vec(), &mut stdin_a);
        stdout_a.push_front(j);
        let mut stdout_b = execute_program(&mut program.to_vec(), &mut stdout_a);
        stdout_b.push_front(k);
        let mut stdout_c = execute_program(&mut program.to_vec(), &mut stdout_b);
        stdout_c.push_front(l);
        let mut stdout_d = execute_program(&mut program.to_vec(), &mut stdout_c);
        stdout_d.push_front(m);
        let mut stdout_e = execute_program(&mut program.to_vec(), &mut stdout_d);
        let signal = stdout_e.pop_front().unwrap();
        highest_signal = cmp::max(signal, highest_signal);
    }

    println!("{}", highest_signal);
    Ok(())
}

fn generate_permutations(list: &mut Vec<i32>, n: usize, combinations: &mut Vec<Vec<i32>>) {
    if n == 1 {
        combinations.push(list.to_vec());
    } else {
        for i in 0..n - 1 {
            generate_permutations(list, n - 1, combinations);
            if n % 2 == 0 {
                list.swap(i, n - 1);
            } else {
                list.swap(0, n - 1);
            }
        }
        generate_permutations(list, n - 1, combinations);
    }
}

fn execute_program(program: &mut [i32], stdin: &mut VecDeque<i32>) -> VecDeque<i32> {
    let mut stdout = VecDeque::new();
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
                let input = stdin.pop_front().unwrap();
                let arg1 = get_arg(program, pc, 1);
                pc += 1;
                program[arg1 as usize] = input;
            }
            4 => {
                let mode = get_digit(opcode_number, 2);
                let arg1 = get_arg(program, pc, mode);
                pc += 1;
                stdout.push_back(arg1);
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
        };
    }
    stdout
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
