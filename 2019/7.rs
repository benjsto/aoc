use std::collections::HashSet;
use std::io;
use std::io::Write;

#[derive(PartialEq,Debug)]
enum ParameterMode {
    Immediate,
    Position,
}

#[derive(PartialEq,Debug)]
struct Instruction {
    code: i32,
    p1_mode: ParameterMode,
    p2_mode: ParameterMode,
    p3_mode: ParameterMode,
    p1: i32,
    p2: i32,
    p3: i32,
    size: usize,
}

fn get_program_from_string(string:String) -> Vec<i32> {
    let mut program:Vec<i32> = vec![];

    for token in string.split(","){
        program.push(token.parse::<i32>().unwrap())
     }

    return program;
}

fn execute_program(program:Vec<i32>, inputs: Vec<String>) -> (Vec<i32>, Vec<String>) {
    let mut new_program = program.clone();

    let mut pc: usize = 0;
    let mut current_instruction: i32 = 0;

    let mut inputs = inputs.iter();
    let mut outputs = vec![];

    while current_instruction != 99 {
        let instruction = get_instruction_at_index(pc, &new_program);

        current_instruction = instruction.code;
        pc = pc + instruction.size;

        let mut input = None;

        if current_instruction == 3 {
            input = inputs.next();
        }

        let (executed_program, jump_address, output) = execute_instruction(instruction, &new_program, input);

        if let Some(out) = output {
            outputs.push(out);
        }

        new_program = executed_program;

        match jump_address {
            None => (),
            Some(counter) => pc = counter as usize,
        };
    }

    return (new_program, outputs);
}

trait Input {
    fn get_input_from_stdin(&self) -> String {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_n) => (),
            Err(error) => println!("error: {}", error),
        };

        return input.trim_end().to_string();
    }
}

trait Output {
    fn write_string_to_stdout(&self, output:String) {
        match io::stdout().write_fmt(format_args!("output: {:}\n", output)) {
            Ok(_n) => (),
            Err(error) => println!("error: {}", error),
        };

        match io::stdout().flush() {
            Ok(_n) => (),
            Err(error) => println!("error: {}", error),
        };
    }
}

fn execute_instruction(instruction:Instruction, program:&Vec<i32>, input:Option<&String>) -> (Vec<i32>, Option<i32>, Option<String>) {
    let mut new_program = program.clone();

    match instruction.code {
        1 => {
            // Add
            let op1 = if instruction.p1_mode == ParameterMode::Position {program[instruction.p1 as usize]} else {instruction.p1};
            let op2 = if instruction.p2_mode == ParameterMode::Position {program[instruction.p2 as usize]} else {instruction.p2};
            let val = op1 + op2;

            new_program[instruction.p3 as usize] = val;
        },
        2 => {
            // Multiply
            let op1 = if instruction.p1_mode == ParameterMode::Position {program[instruction.p1 as usize]} else {instruction.p1};
            let op2 = if instruction.p2_mode == ParameterMode::Position {program[instruction.p2 as usize]} else {instruction.p2};

            let val = op1 * op2;

            new_program[instruction.p3 as usize] = val;
        },
        3 => {
            // Input
            if let Some(i) = input {
                let val = i.parse::<i32>().unwrap();

                new_program[instruction.p1 as usize] = val;
            }
        },
        4 => {
            // Output
            let output = if instruction.p1_mode == ParameterMode::Position {program[instruction.p1 as usize]} else {instruction.p1};

            // write_output(output.to_string());
            return (new_program, None, Some(output.to_string()))
        },
        5 => {
            // Jump-if-true
            let p1 = if instruction.p1_mode == ParameterMode::Position {program[instruction.p1 as usize]} else {instruction.p1};
            let p2 = if instruction.p2_mode == ParameterMode::Position {program[instruction.p2 as usize]} else {instruction.p2};

            if p1 != 0 {
                return (new_program, Some(p2), None);
            }
        },
        6 => {
            // Jump-if-false
            let p1 = if instruction.p1_mode == ParameterMode::Position {program[instruction.p1 as usize]} else {instruction.p1};
            let p2 = if instruction.p2_mode == ParameterMode::Position {program[instruction.p2 as usize]} else {instruction.p2};

            if p1 == 0 {
                return (new_program, Some(p2), None);
            }
        },
        7 => {
            // Less-than
            let p1 = if instruction.p1_mode == ParameterMode::Position {program[instruction.p1 as usize]} else {instruction.p1};
            let p2 = if instruction.p2_mode == ParameterMode::Position {program[instruction.p2 as usize]} else {instruction.p2};

            let val = if p1 < p2 {1} else {0};

            new_program[instruction.p3 as usize] = val;
        },
        8 => {
            // Equals
            let p1 = if instruction.p1_mode == ParameterMode::Position {program[instruction.p1 as usize]} else {instruction.p1};
            let p2 = if instruction.p2_mode == ParameterMode::Position {program[instruction.p2 as usize]} else {instruction.p2};

            let val = if p1 == p2 {1} else {0};

            new_program[instruction.p3 as usize] = val;
        }
        _ => (),
    };

    return (new_program, None, None);
}

fn parse_instruction_code(instruction:i32) -> (ParameterMode, ParameterMode, ParameterMode, i32) {
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

    let p3_mode = if instruction_str.chars().nth(0).unwrap() == '1' {ParameterMode::Immediate} else {ParameterMode::Position};
    let p2_mode = if instruction_str.chars().nth(1).unwrap() == '1' {ParameterMode::Immediate} else {ParameterMode::Position};
    let p1_mode = if instruction_str.chars().nth(2).unwrap() == '1' {ParameterMode::Immediate} else {ParameterMode::Position};

    let intcode = format!("{}{}", instruction_str.chars().nth(3).unwrap(), instruction_str.chars().nth(4).unwrap()).parse::<i32>().unwrap();

    return (p1_mode, p2_mode, p3_mode, intcode);
}

fn get_instruction_at_index(idx:usize, program:&Vec<i32>) -> Instruction {
    let (p1_mode, p2_mode, p3_mode, opcode) = parse_instruction_code(program[idx]);

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
        instruction.p1 = program[idx+1];
        instruction.p2 = program[idx+2];
        instruction.p3 = program[idx+3];
    } else if [5,6].contains(&instruction.code) {
        instruction.p1 = program[idx+1];
        instruction.p2 = program[idx+2];
        instruction.size = 3;
    } else if [3,4].contains(&instruction.code) {
        instruction.p1 = program[idx+1];
        instruction.size = 2;
    } else if [99].contains(&instruction.code) {
        instruction.size = 1;
    }

    return instruction;
}

fn run_program_in_amplifiers(program:Vec<i32>, phases:[i32;5]) -> i32 {
    let mut output = 0;

    for a in 0..5 {
        let inputs = vec![phases[a].to_string(), output.to_string()];

        let (_, program_output) = execute_program(program.clone(), inputs);

        if program_output.len() > 0 {
            output = program_output[0].parse::<i32>().unwrap();
        }
    }

    return output;
}

fn main() -> io::Result<()> {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_n) => (),
        Err(error) => println!("error: {}", error),
    }

    let program = get_program_from_string(input.trim_end().to_string());

    let mut max_output = 0;
    let mut max_phases = [0,0,0,0,0];

    for p0 in 0..5 {
        for p1 in 0..5 {
            for p2 in 0..5 {
                for p3 in 0..5 {
                    for p4 in 0..5 {
                        let phases = [p0,p1,p2,p3,p4];
                        let phases_set: HashSet<i32> = phases.iter().cloned().collect();

                        if phases_set.len() < 5 {
                            continue;
                        }

                        let output = run_program_in_amplifiers(program.clone(), phases);

                        if output > max_output {
                            max_output = output;
                            max_phases = [p0,p1,p2,p3,p4];
                        }
                    }
                }
            }
        }
    }

    println!("max_output: {:?}", max_output);
    println!("max_phases: {:?}", max_phases);

    Ok(())
}

#[cfg(test)]
mod tests {
    use *;

    #[test]
    fn test_run_program_in_amplifiers() {

    }
}
