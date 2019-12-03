use std::io;

struct Instruction {
    code: i32,
    input1_idx: usize,
    input2_idx: usize,
    output_idx: usize,
}

fn get_program_from_string(string:String) -> Vec<i32> {
    let mut program:Vec<i32> = vec![];

    for token in string.split(","){
        program.push(token.parse::<i32>().unwrap())
     }

    return program;
}

fn execute_program(program:Vec<i32>) -> Vec<i32> {
    let mut new_program = program.clone();

    let mut pc: usize = 0;
    let mut current_instruction: i32 = 0;

    while current_instruction != 99 {
        let instruction = get_instruction_at_index(pc, &new_program);

        current_instruction = instruction.code;

        new_program = execute_instruction(instruction, &new_program);

        pc = pc + 4;
    }

    return new_program;
}

fn execute_instruction(instruction:Instruction, program:&Vec<i32>) -> Vec<i32> {
    let mut new_program = program.clone();

    match instruction.code {
        1 => {
            let op1 = program[instruction.input1_idx];
            let op2 = program[instruction.input2_idx];
            let val = op1 + op2;

            new_program[instruction.output_idx] = val;
        },
        2 => {
            let op1 = program[instruction.input1_idx];
            let op2 = program[instruction.input2_idx];
            let val = op1 * op2;

            new_program[instruction.output_idx] = val;
        },
        _ => (),
    };

    return new_program;
}

fn get_instruction_at_index(idx:usize, program:&Vec<i32>) -> Instruction {
    let mut instruction: Instruction = Instruction{
        code: program[idx],
        input1_idx: 0,
        input2_idx: 0,
        output_idx: 0,
    };

    if [1,2].contains(&instruction.code) {
        instruction.input1_idx = program[idx+1] as usize;
        instruction.input2_idx = program[idx+2] as usize;
        instruction.output_idx = program[idx+3] as usize;
    }

    return instruction;
}

fn modify_program_value_at_index(idx:usize, val: i32, program:Vec<i32>) -> Vec<i32> {
    let mut new_program = program.clone();

    new_program[idx] = val;

    return new_program;
}

fn main() -> io::Result<()> {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_n) => (),
        Err(error) => println!("error: {}", error),
    }

    let program = get_program_from_string(input.trim_end().to_string());

    let (mut noun, mut verb, mut output) = (0, 0, 0);

    while noun < 100 {
        verb = 0;
        while verb < 100 {
            let new_program = program.clone();

            let new_program = modify_program_value_at_index(1, noun, new_program);
            let new_program = modify_program_value_at_index(2, verb, new_program);
            let new_program = execute_program(new_program);

            output = new_program[0];

            if output == 19690720 {
                break;
            }

            verb = verb + 1;
        }

        if output == 19690720 {
            break;
        }

        noun = noun + 1;
    }

    // let program = execute_program(program);

    println!("noun: {}", noun);
    println!("verb: {}", verb);

    println!("100 * noun + verb = {}", 100 * noun + verb);

    Ok(())
}

#[cfg(test)]
mod tests {
    use *;

    #[test]
    fn test_get_instruction_at_index() {
        let instruction = get_instruction_at_index(4, &vec![0,2,4,5,1,2,3,4]);

        assert_eq!(instruction.code, 1);
        assert_eq!(instruction.input1_idx, 2);
        assert_eq!(instruction.input2_idx, 3);
        assert_eq!(instruction.output_idx, 4);
    }

    #[test]
    fn test_execute_instruction() {
        let program = vec![1,0,0,0,99];
        let instruction = Instruction{code: 1, input1_idx: 0, input2_idx: 0, output_idx: 0};

        let new_program = execute_instruction(instruction, &program);

        assert_eq!(new_program, vec![2,0,0,0,99]);

        let program = vec![2,3,0,3,99];
        let instruction = Instruction{code: 2, input1_idx: 3, input2_idx: 0, output_idx: 3};

        let new_program = execute_instruction(instruction, &program);

        assert_eq!(new_program, vec![2,3,0,6,99]);

        let program = vec![2,4,4,5,99,0];
        let instruction = Instruction{code: 2, input1_idx: 4, input2_idx: 4, output_idx: 5};

        let new_program = execute_instruction(instruction, &program);

        assert_eq!(new_program, vec![2,4,4,5,99,9801]);
    }

    #[test]
    fn test_execute_program() {
        let program = execute_program(vec![1,1,1,4,99,5,6,0,99]);

        assert_eq!(program, vec![30,1,1,4,2,5,6,0,99]);
    }

    #[test]
    fn test_modify_program() {
        let program = vec![2,4,4,5,99,0];

        let new_program = modify_program_value_at_index(5, 5, program);

        assert_eq!(new_program, vec![2,4,4,5,99,5]);
    }
}
