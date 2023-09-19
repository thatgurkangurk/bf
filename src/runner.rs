use std::io::Read;

use crate::instructions::Instruction;

pub fn run(instructions: &Vec<Instruction>, tape: &mut Vec<u8>, data_ptr: &mut usize) {
    for instruction in instructions {
        match instruction {
            Instruction::Loop(nested) => {
                while tape[*data_ptr] != 0 {
                    run(nested, tape, data_ptr)
                }
            }
            Instruction::Write => print!("{}", tape[*data_ptr] as char),
            Instruction::Read => {
                let mut input: [u8; 1] = [0; 1];
                std::io::stdin()
                    .read_exact(&mut input)
                    .expect("something went wrong while reading stdin");
                tape[*data_ptr] = input[0];
            }
            Instruction::IncrementPointer => *data_ptr += 1,
            Instruction::DecrementPointer => *data_ptr -= 1,
            Instruction::Increment => tape[*data_ptr] += 1,
            Instruction::Decrement => tape[*data_ptr] -= 1,
        }
    }
}
