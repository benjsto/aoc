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

fn execute_program(program:Vec<i32>, get_input: &dyn Fn()->String, write_output: &dyn Fn(String)) -> Vec<i32> {
    let mut new_program = program.clone();

    let mut pc: usize = 0;
    let mut current_instruction: i32 = 0;

    while current_instruction != 99 {
        let instruction = get_instruction_at_index(pc, &new_program);

        current_instruction = instruction.code;
        pc = pc + instruction.size;

        let (executed_program, jump_address) = execute_instruction(instruction, &new_program, get_input, write_output);

        new_program = executed_program;

        match jump_address {
            None => (),
            Some(counter) => pc = counter as usize,
        };
    }

    return new_program;
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

fn execute_instruction(instruction:Instruction, program:&Vec<i32>, get_input: impl Fn()->String, write_output: impl Fn(String)) -> (Vec<i32>, Option<i32>) {
    let mut new_program = program.clone();

    // println!("instruction: {:?}", instruction);

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
            let input = get_input();

            let val = input.parse::<i32>().unwrap();

            new_program[instruction.p1 as usize] = val;
        },
        4 => {
            // Output
            let output = if instruction.p1_mode == ParameterMode::Position {program[instruction.p1 as usize]} else {instruction.p1};

            write_output(output.to_string());
        },
        5 => {
            // Jump-if-true
            let p1 = if instruction.p1_mode == ParameterMode::Position {program[instruction.p1 as usize]} else {instruction.p1};
            let p2 = if instruction.p2_mode == ParameterMode::Position {program[instruction.p2 as usize]} else {instruction.p2};

            if p1 != 0 {
                return (new_program, Some(p2));
            }
        },
        6 => {
            // Jump-if-false
            let p1 = if instruction.p1_mode == ParameterMode::Position {program[instruction.p1 as usize]} else {instruction.p1};
            let p2 = if instruction.p2_mode == ParameterMode::Position {program[instruction.p2 as usize]} else {instruction.p2};

            if p1 == 0 {
                return (new_program, Some(p2));
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

    return (new_program, None);
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
    fn test_parse_instruction_code() {
        let (p1_mode, p2_mode, p3_mode, opcode) = parse_instruction_code(1001);

        assert_eq!(p1_mode, ParameterMode::Position);
        assert_eq!(p2_mode, ParameterMode::Immediate);
        assert_eq!(p3_mode, ParameterMode::Position);
        assert_eq!(opcode, 1);

        let (p1_mode, p2_mode, p3_mode, opcode) = parse_instruction_code(11120);

        assert_eq!(p1_mode, ParameterMode::Immediate);
        assert_eq!(p2_mode, ParameterMode::Immediate);
        assert_eq!(p3_mode, ParameterMode::Immediate);
        assert_eq!(opcode, 20);

        let (p1_mode, p2_mode, p3_mode, opcode) = parse_instruction_code(3);

        assert_eq!(p1_mode, ParameterMode::Position);
        assert_eq!(p2_mode, ParameterMode::Position);
        assert_eq!(p3_mode, ParameterMode::Position);
        assert_eq!(opcode, 3);
    }

    #[test]
    fn test_execute_program() {
        let mock_get_input = || { "8".to_string() };
        let mock_output = |out:String| { assert_eq!(out, "1".to_string()) };
        execute_program(vec![3,9,8,9,10,9,4,9,99,-1,8], &mock_get_input, &mock_output);

        let mock_get_input = || { "7".to_string() };
        let mock_output = |out:String| { assert_eq!(out, "0".to_string()) };
        execute_program(vec![3,9,8,9,10,9,4,9,99,-1,8], &mock_get_input, &mock_output);

        let mock_get_input = || { "7".to_string() };
        let mock_output = |out:String| { assert_eq!(out, "1".to_string()) };
        execute_program(vec![3,9,7,9,10,9,4,9,99,-1,8], &mock_get_input, &mock_output);

        let mock_get_input = || { "8".to_string() };
        let mock_output = |out:String| { assert_eq!(out, "0".to_string()) };
        execute_program(vec![3,9,7,9,10,9,4,9,99,-1,8], &mock_get_input, &mock_output);

        let mock_get_input = || { "8".to_string() };
        let mock_output = |out:String| { assert_eq!(out, "1".to_string()) };
        execute_program(vec![3,3,1108,-1,8,3,4,3,99], &mock_get_input, &mock_output);

        let mock_get_input = || { "9".to_string() };
        let mock_output = |out:String| { assert_eq!(out, "0".to_string()) };
        execute_program(vec![3,3,1108,-1,8,3,4,3,99], &mock_get_input, &mock_output);

        let mock_get_input = || { "7".to_string() };
        let mock_output = |out:String| { assert_eq!(out, "1".to_string()) };
        execute_program(vec![3,3,1107,-1,8,3,4,3,99], &mock_get_input, &mock_output);

        let mock_get_input = || { "10".to_string() };
        let mock_output = |out:String| { assert_eq!(out, "0".to_string()) };
        execute_program(vec![3,3,1107,-1,8,3,4,3,99], &mock_get_input, &mock_output);

        let mock_get_input = || { "0".to_string() };
        let mock_output = |out:String| { assert_eq!(out, "0".to_string()) };
        execute_program(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], &mock_get_input, &mock_output);

        let mock_get_input = || { "5".to_string() };
        let mock_output = |out:String| { assert_eq!(out, "1".to_string()) };
        execute_program(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], &mock_get_input, &mock_output);

        let mock_get_input = || { "0".to_string() };
        let mock_output = |out:String| { assert_eq!(out, "0".to_string()) };
        execute_program(vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], &mock_get_input, &mock_output);

        let mock_get_input = || { "5".to_string() };
        let mock_output = |out:String| { assert_eq!(out, "1".to_string()) };
        execute_program(vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], &mock_get_input, &mock_output);

        let mock_get_input = || { "7".to_string() };
        let mock_output = |out:String| { assert_eq!(out, "999".to_string()) };
        execute_program(vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], &mock_get_input, &mock_output);
    }
}
