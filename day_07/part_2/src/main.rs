use std::cmp;
use std::io;
use std::sync::mpsc;
use std::thread;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let program: Vec<i32> = buffer
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut vec = vec![5, 6, 7, 8, 9];
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
        let (tx_input, mut rx_a) = mpsc::channel();
        let (mut tx_a, mut rx_b) = mpsc::channel();
        let (mut tx_b, mut rx_c) = mpsc::channel();
        let (mut tx_c, mut rx_d) = mpsc::channel();
        let (mut tx_d, mut rx_e) = mpsc::channel();
        let (mut tx_e, rx_output) = mpsc::channel();

        let mut program_a = program.to_vec();
        let mut program_b = program.to_vec();
        let mut program_c = program.to_vec();
        let mut program_d = program.to_vec();
        let mut program_e = program.to_vec();

        tx_input.send(i).unwrap();
        tx_a.send(j).unwrap();
        tx_b.send(k).unwrap();
        tx_c.send(l).unwrap();
        tx_d.send(m).unwrap();

        tx_input.send(0).unwrap();

        let mut threads = Vec::new();
        threads.push(thread::spawn(move || {
            execute_program(&mut program_a, &mut rx_a, &mut tx_a);
        }));
        threads.push(thread::spawn(move || {
            execute_program(&mut program_b, &mut rx_b, &mut tx_b);
        }));
        threads.push(thread::spawn(move || {
            execute_program(&mut program_c, &mut rx_c, &mut tx_c);
        }));
        threads.push(thread::spawn(move || {
            execute_program(&mut program_d, &mut rx_d, &mut tx_d);
        }));
        threads.push(thread::spawn(move || {
            execute_program(&mut program_e, &mut rx_e, &mut tx_e);
        }));

        let mut signal = 0;
        while let Ok(output) = rx_output.recv() {
            signal = output;
            let _ = tx_input.send(output);
        }

        highest_signal = cmp::max(signal, highest_signal);

        for thread in threads {
            thread.join().unwrap();
        }
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

fn execute_program(
    program: &mut [i32],
    stdin: &mut mpsc::Receiver<i32>,
    stdout: &mut mpsc::Sender<i32>,
) {
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
                let input = stdin.recv().expect("could not recv");
                let arg1 = get_arg(program, pc, 1);
                pc += 1;
                program[arg1 as usize] = input;
            }
            4 => {
                let mode = get_digit(opcode_number, 2);
                let arg1 = get_arg(program, pc, mode);
                pc += 1;
                stdout.send(arg1).expect("could not send");
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
