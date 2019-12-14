use std::collections::HashMap;
use std::io;
use std::io::Write;

#[derive(PartialEq,Debug)]
enum ParameterMode {
    Immediate,
    Position,
    Relative,
}

#[derive(PartialEq,Debug)]
struct Instruction {
    code: i64,
    p1_mode: ParameterMode,
    p2_mode: ParameterMode,
    p3_mode: ParameterMode,
    p1: i64,
    p2: i64,
    p3: i64,
    size: usize,
}

fn get_program_from_string(string:String) -> Vec<i64> {
    let mut program:Vec<i64> = vec![];

    for token in string.split(","){
        program.push(token.parse::<i64>().unwrap())
     }

    return program;
}

fn execute_program(program:Vec<i64>, get_input: &dyn Fn()->String, write_output: &dyn Fn(String)) -> Vec<i64> {
    let mut program_copy = program.clone();

    let mut pc: usize = 0;
    let mut current_instruction: i64 = 0;
    let mut base: i64 = 0;

    let mut large_memory = HashMap::new();

    while current_instruction != 99 {
        let instruction = get_instruction_at_index(pc, &program_copy, &large_memory);

        current_instruction = instruction.code;
        pc = pc + instruction.size;

        let (executed_program, large_memory_copy, jump_address, relative_base) = execute_instruction(instruction, &program_copy, &large_memory, base, get_input, write_output);

        program_copy = executed_program;
        large_memory = large_memory_copy;

        match relative_base {
            None => (),
            Some(b) => base = base + b,
        };

        match jump_address {
            None => (),
            Some(counter) => pc = counter as usize,
        };
    }

    return program_copy;
}

fn get_input_from_stdin() -> String {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_n) => (),
        Err(error) => println!("error: {}", error),
    };

    return input.trim_end().to_string();
}

fn write_string_to_stdout(output:String) {
    match io::stdout().write_fmt(format_args!("output: {:}\n", output)) {
        Ok(_n) => (),
        Err(error) => println!("error: {}", error),
    };

    match io::stdout().flush() {
        Ok(_n) => (),
        Err(error) => println!("error: {}", error),
    };
}

fn read_from_memory(program:&Vec<i64>, large_memory:&HashMap<usize,i64>, address:usize) -> i64 {
    let program_length = program.len();

    if address >= program_length {
        if large_memory.contains_key(&address) {
            return large_memory[&address];
        }

        return 0;
    }

    program[address]
}

fn write_to_memory(program_in:&Vec<i64>, large_memory_in:&HashMap<usize,i64>, address:usize, value:i64) -> (Vec<i64>, HashMap<usize,i64>) {
    let program_length = program_in.len();

    let mut program_copy = program_in.clone();
    let mut large_memory_copy = large_memory_in.clone();

    if address >= program_length {
        large_memory_copy.insert(address, value);
    } else {
        program_copy[address] = value;
    }

    return (program_copy, large_memory_copy)
}

fn execute_instruction(instruction:Instruction, program:&Vec<i64>, large_memory:&HashMap<usize,i64>, base:i64, get_input: impl Fn()->String, write_output: impl Fn(String)) -> (Vec<i64>, HashMap<usize,i64>, Option<i64>, Option<i64>) {
    let mut program_copy = program.clone();
    let mut large_memory_copy = large_memory.clone();

    match instruction.code {
        1 => {
            // Add
            let op1 = match instruction.p1_mode {
                ParameterMode::Immediate => instruction.p1,
                ParameterMode::Position => read_from_memory(program, large_memory, instruction.p1 as usize),
                ParameterMode::Relative => read_from_memory(program, large_memory, (base + instruction.p1) as usize),
            };

            let op2 = match instruction.p2_mode {
                ParameterMode::Immediate => instruction.p2,
                ParameterMode::Position => read_from_memory(program, large_memory, instruction.p2 as usize),
                ParameterMode::Relative => read_from_memory(program, large_memory, (base + instruction.p2) as usize),
            };

            let val = op1 + op2;

            let address = match instruction.p3_mode {
                ParameterMode::Immediate => instruction.p3 as usize,
                ParameterMode::Position => instruction.p3 as usize,
                ParameterMode::Relative => (base + instruction.p3) as usize,
            };

            let (new_program_copy, new_large_memory_copy) = write_to_memory(&program_copy, &large_memory_copy, address, val);

            program_copy = new_program_copy;
            large_memory_copy = new_large_memory_copy;
        },
        2 => {
            // Multiply
            let op1 = match instruction.p1_mode {
                ParameterMode::Immediate => instruction.p1,
                ParameterMode::Position => read_from_memory(program, large_memory, instruction.p1 as usize),
                ParameterMode::Relative => read_from_memory(program, large_memory, (base + instruction.p1) as usize),
            };

            let op2 = match instruction.p2_mode {
                ParameterMode::Immediate => instruction.p2,
                ParameterMode::Position => read_from_memory(program, large_memory, instruction.p2 as usize),
                ParameterMode::Relative => read_from_memory(program, large_memory, (base + instruction.p2) as usize),
            };

            let address = match instruction.p3_mode {
                ParameterMode::Immediate => instruction.p3 as usize,
                ParameterMode::Position => instruction.p3 as usize,
                ParameterMode::Relative => (base + instruction.p3) as usize,
            };

            let val = op1 * op2;

            let (new_program_copy, new_large_memory_copy) = write_to_memory(&program_copy, &large_memory_copy, address, val);

            program_copy = new_program_copy;
            large_memory_copy = new_large_memory_copy;
        },
        3 => {
            // Input
            let input = get_input();
            let val = input.parse::<i64>().unwrap();

            let address = match instruction.p1_mode {
                ParameterMode::Immediate => (base + instruction.p1) as usize,
                ParameterMode::Position => instruction.p1 as usize,
                ParameterMode::Relative => (base + instruction.p1) as usize,
            };

            // println!("writing input value {} to {}", val, address);

            let (new_program_copy, new_large_memory_copy) = write_to_memory(&program_copy, &large_memory_copy, address, val);

            program_copy = new_program_copy;
            large_memory_copy = new_large_memory_copy;
        },
        4 => {
            // Output
            let output = match instruction.p1_mode {
                ParameterMode::Immediate => instruction.p1,
                ParameterMode::Position => read_from_memory(program, large_memory, instruction.p1 as usize),
                ParameterMode::Relative => read_from_memory(program, large_memory, (base + instruction.p1) as usize),
            };

            write_output(output.to_string());
        },
        5 => {
            // Jump-if-true
            let p1 = match instruction.p1_mode {
                ParameterMode::Immediate => instruction.p1,
                ParameterMode::Position => read_from_memory(program, large_memory, instruction.p1 as usize),
                ParameterMode::Relative => read_from_memory(program, large_memory, (base + instruction.p1) as usize),
            };

            let p2 = match instruction.p2_mode {
                ParameterMode::Immediate => instruction.p2,
                ParameterMode::Position => read_from_memory(program, large_memory, instruction.p2 as usize),
                ParameterMode::Relative => read_from_memory(program, large_memory, (base + instruction.p2) as usize),
            };

            // println!("if {} is true, jump to {}", p1, p2);

            if p1 != 0 {
                return (program_copy, large_memory_copy, Some(p2), None);
            }
        },
        6 => {
            // Jump-if-false
            let p1 = match instruction.p1_mode {
                ParameterMode::Immediate => instruction.p1,
                ParameterMode::Position => read_from_memory(program, large_memory, instruction.p1 as usize),
                ParameterMode::Relative => read_from_memory(program, large_memory, (base + instruction.p1) as usize),
            };

            let p2 = match instruction.p2_mode {
                ParameterMode::Immediate => instruction.p2,
                ParameterMode::Position => read_from_memory(program, large_memory, instruction.p2 as usize),
                ParameterMode::Relative => read_from_memory(program, large_memory, (base + instruction.p2) as usize),
            };

            if p1 == 0 {
                return (program_copy, large_memory_copy, Some(p2), None);
            }
        },
        7 => {
            // Less-than
            let p1 = match instruction.p1_mode {
                ParameterMode::Immediate => instruction.p1,
                ParameterMode::Position => read_from_memory(program, large_memory, instruction.p1 as usize),
                ParameterMode::Relative => read_from_memory(program, large_memory, (base + instruction.p1) as usize),
            };

            let p2 = match instruction.p2_mode {
                ParameterMode::Immediate => instruction.p2,
                ParameterMode::Position => read_from_memory(program, large_memory, instruction.p2 as usize),
                ParameterMode::Relative => read_from_memory(program, large_memory, (base + instruction.p2) as usize),
            };

            let address = match instruction.p3_mode {
                ParameterMode::Immediate => instruction.p3 as usize,
                ParameterMode::Position => instruction.p3 as usize,
                ParameterMode::Relative => (base + instruction.p3) as usize,
            };

            let val = if p1 < p2 {1} else {0};

            let (new_program_copy, new_large_memory_copy) = write_to_memory(&program_copy, &large_memory_copy, address, val);

            program_copy = new_program_copy;
            large_memory_copy = new_large_memory_copy;
        },
        8 => {
            // Equals
            let p1 = match instruction.p1_mode {
                ParameterMode::Immediate => instruction.p1,
                ParameterMode::Position => read_from_memory(program, large_memory, instruction.p1 as usize),
                ParameterMode::Relative => read_from_memory(program, large_memory, (base + instruction.p1) as usize),
            };

            let p2 = match instruction.p2_mode {
                ParameterMode::Immediate => instruction.p2,
                ParameterMode::Position => read_from_memory(program, large_memory, instruction.p2 as usize),
                ParameterMode::Relative => read_from_memory(program, large_memory, (base + instruction.p2) as usize),
            };

            let address = match instruction.p3_mode {
                ParameterMode::Immediate => instruction.p3 as usize,
                ParameterMode::Position => instruction.p3 as usize,
                ParameterMode::Relative => (base + instruction.p3) as usize,
            };

            let val = if p1 == p2 {1} else {0};

            let (new_program_copy, new_large_memory_copy) = write_to_memory(&program_copy, &large_memory_copy, address, val);

            program_copy = new_program_copy;
            large_memory_copy = new_large_memory_copy;
        },
        9 => {
            // Relative base
            let p1 = match instruction.p1_mode {
                ParameterMode::Immediate => instruction.p1,
                ParameterMode::Position => read_from_memory(program, large_memory, instruction.p1 as usize),
                ParameterMode::Relative => read_from_memory(program, large_memory, (base + instruction.p1) as usize),
            };

            return (program_copy, large_memory_copy, None, Some(p1));
        }
        _ => (),
    };

    return (program_copy, large_memory_copy, None, None);
}

fn parse_instruction_code(instruction:i64) -> (ParameterMode, ParameterMode, ParameterMode, i64) {
    let mut instruction_str = instruction.to_string();

    let num_chars = instruction_str.chars().count();

    if [1,2].contains(&num_chars) {
        return (ParameterMode::Position, ParameterMode::Position, ParameterMode::Position, instruction);
    }

    if num_chars == 3 {
        instruction_str = format!("{}{}", "00", instruction_str);
    }

    if num_chars == 4 {
        instruction_str = format!("{}{}", "0", instruction_str);
    }

    let p3_mode = match instruction_str.chars().nth(0).unwrap() {
        '0' => ParameterMode::Position,
        '1' => ParameterMode::Immediate,
        '2' => ParameterMode::Relative,
        _ => ParameterMode::Position,
    };

    let p2_mode = match instruction_str.chars().nth(1).unwrap() {
        '0' => ParameterMode::Position,
        '1' => ParameterMode::Immediate,
        '2' => ParameterMode::Relative,
        _ => ParameterMode::Position,
    };

    let p1_mode = match instruction_str.chars().nth(2).unwrap() {
        '0' => ParameterMode::Position,
        '1' => ParameterMode::Immediate,
        '2' => ParameterMode::Relative,
        _ => ParameterMode::Position,
    };

    let intcode = format!("{}{}", instruction_str.chars().nth(3).unwrap(), instruction_str.chars().nth(4).unwrap()).parse::<i64>().unwrap();

    return (p1_mode, p2_mode, p3_mode, intcode);
}

fn get_instruction_at_index(idx:usize, program:&Vec<i64>, large_memory:&HashMap<usize,i64>) -> Instruction {
    let code = read_from_memory(program, large_memory, idx);

    let (p1_mode, p2_mode, p3_mode, opcode) = parse_instruction_code(code);

    let mut instruction: Instruction = Instruction{
        code: opcode,
        p1: 0,
        p2: 0,
        p3: 0,
        p1_mode: p1_mode,
        p2_mode: p2_mode,
        p3_mode: p3_mode,
        size: 4,
    };

    if [1,2,7,8].contains(&instruction.code) {
        instruction.p1 = read_from_memory(program, large_memory, idx+1);
        instruction.p2 = read_from_memory(program, large_memory, idx+2);
        instruction.p3 = read_from_memory(program, large_memory, idx+3);
    } else if [5,6].contains(&instruction.code) {
        instruction.p1 = read_from_memory(program, large_memory, idx+1);
        instruction.p2 = read_from_memory(program, large_memory, idx+2);
        instruction.size = 3;
    } else if [3,4,9].contains(&instruction.code) {
        instruction.p1 = read_from_memory(program, large_memory, idx+1);
        instruction.size = 2;
    } else if [99].contains(&instruction.code) {
        instruction.size = 1;
    }

    return instruction;
}

fn main() -> io::Result<()> {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_n) => (),
        Err(error) => println!("error: {}", error),
    }

    let program = get_program_from_string(input.trim_end().to_string());

    execute_program(program, &get_input_from_stdin, &write_string_to_stdout);

    Ok(())
}

#[cfg(test)]
mod tests {
    use *;

    #[test]
    fn test_execute_program() {

    }
}
