#[derive(Debug, Clone)]
// operation codes that the lexer determined
pub enum OpCode {
    Write,
    Read,
    Increment,
    Decrement,
    IncrementPointer,
    DecrementPointer,
    LoopBegin,
    LoopEnd,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Write,
    Read,
    Increment,
    Decrement,
    IncrementPointer,
    DecrementPointer,
    Loop(Vec<Instruction>),
}
