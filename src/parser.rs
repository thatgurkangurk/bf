use crate::instructions::{Instruction, OpCode};

pub fn parse(opcodes: Vec<OpCode>) -> Vec<Instruction> {
    let mut program: Vec<Instruction> = Vec::new();
    let mut start = 0;
    let mut stack = 0;

    for (i, operation) in opcodes.iter().enumerate() {
        if stack == 0 {
            let instruction = match operation {
                OpCode::Write => Some(Instruction::Write),
                OpCode::Read => Some(Instruction::Read),
                OpCode::Increment => Some(Instruction::Increment),
                OpCode::Decrement => Some(Instruction::Decrement),
                OpCode::IncrementPointer => Some(Instruction::IncrementPointer),
                OpCode::DecrementPointer => Some(Instruction::DecrementPointer),

                OpCode::LoopBegin => {
                    stack += 1;
                    start = i;
                    None
                }

                OpCode::LoopEnd => panic!("loop without beginning ending at #{}", i),
            };

            match instruction {
                Some(instruction) => program.push(instruction),
                None => (),
            }
        } else {
            match operation {
                OpCode::LoopEnd => {
                    stack -= 1;

                    if stack == 0 {
                        program.push(Instruction::Loop(parse(opcodes[start + 1..i].to_vec())));
                    }
                }
                OpCode::LoopBegin => {
                    stack += 1;
                }
                _ => (),
            }
        }
    }

    if stack != 0 {
        panic!("loop starts at #{}, but has no matching ending", start);
    }

    return program;
}
