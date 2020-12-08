use std::str::FromStr;

static PROGRAM_DATA: &'static str = include_str!("../data/data_08.txt");

fn main() {
    let program = PROGRAM_DATA.lines()
        .map(|line| Some(parse(line))).collect::<Vec<_>>();

    // Part 1.

    let cpu_state = run(&mut program.clone()[..]);
    println!("Part 1: The cpu accumulator is: {}", cpu_state.acc);


    // Part 2.

    // Bruteforce approach.
    for mod_index in 0..program.len() {
        // Skip this iteration if the instruction opcode is 'acc'.
        if let Some((OpCode::Acc, _)) = program[mod_index] { continue; }

        let mut mod_program = program.clone();

        mod_program[mod_index] = match &mut mod_program[mod_index] {
            Some((OpCode::Nop, arg)) => Some((OpCode::Jmp, *arg)),
            Some((OpCode::Jmp, arg)) => Some((OpCode::Nop, *arg)),
            instr => *instr,
        };

        let modded = mod_program[mod_index].unwrap();

        let cpu_state = run(&mut mod_program);
        if cpu_state.ip == mod_program.len() {
            println!(
                "Part 2: The accumulator is: {} (and the modified instruction was {:?})",
                cpu_state.acc,
                modded
            );
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct CpuState {
    ip: usize,
    acc: i64,
}

#[derive(Copy, Clone, Debug)]
enum OpCode {
    Acc,
    Jmp,
    Nop,
}

// Runs the program until it either ends or an attempt to execute a previously executed instruction.
fn run(code: &mut [Option<(OpCode, i64)>]) -> CpuState {
    let mut cpu = CpuState { ip: 0, acc: 0 };

    loop {
        if cpu.ip >= code.len() { break; }

        match code[cpu.ip].take() {
            Some(instr) => cpu = execute(instr, &cpu),
            None => break,
        }
    }

    cpu
}

fn parse(instr: &str) -> (OpCode, i64) {
    let mut tokens = instr.split_whitespace();
    let op = tokens.next().unwrap();

    let arg = tokens.next()
        .map(|arg| i64::from_str(arg).unwrap())
        .unwrap();

    let op = match op {
        "nop" => OpCode::Nop,
        "acc" => OpCode::Acc,
        "jmp" => OpCode::Jmp,
        _ => panic!(),
    };

    (op, arg)
}

fn execute(instr: (OpCode, i64), state: &CpuState) -> CpuState {
    let (op, arg) = instr;

    let mut result = CpuState { ip: state.ip + 1, acc: state.acc };

    match op {
        OpCode::Nop => (),
        OpCode::Acc => result.acc += arg,
        OpCode::Jmp => result.ip = {
            let ip = (state.ip as i64) + arg;
            assert!(ip >= 0);
            ip as usize
        },
    }

    result
}


