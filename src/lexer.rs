use crate::instructions::OpCode;

pub fn lex(src: String) -> Vec<OpCode> {
    let mut ops = Vec::new();

    for symbol in src.chars() {
        let op = match symbol {
            '[' => Some(OpCode::LoopBegin),
            ']' => Some(OpCode::LoopEnd),
            '.' => Some(OpCode::Write),
            ',' => Some(OpCode::Read),
            '>' => Some(OpCode::IncrementPointer),
            '<' => Some(OpCode::DecrementPointer),
            '+' => Some(OpCode::Increment),
            '-' => Some(OpCode::Decrement),
            _ => None,
        };

        // if a character isn't one of the above, its a comment
        match op {
            Some(op) => ops.push(op),
            None => (),
        }
    }

    return ops;
}
