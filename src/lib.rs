//! welcome to yabfr docs!
//!
//! ## what does this crate do?
//!
//! with this crate, you can execute a brainf**k program with a nice and simple rust API.
//!
//! ## features
//!
//! - [x] execute a brainf**k program from a string
//!
//! ## examples
//!
//! ```
//! use yabfr::run;
//!
//! let output = run("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");
//!
//! println!("{}", output)
//! ```
//!
//! # "issues"
//! - currently, there is no way to pass in "input" to the interpreter, will be fixed later *hopefully*
#[deny(missing_docs)]
use lexer::lex;
use parser::parse;
extern crate wasm_bindgen;

/// this module holds the [OpCode](instructions::OpCode) and [Instruction](instructions::Instruction) enums
pub(crate) mod instructions;
/// this module turns the source string into opcodes
pub(crate) mod lexer;
/// this module deals with parsing the [opcodes](instructions::OpCode) into [instructions](instructions::Instruction)
pub(crate) mod parser;
/// this module runs the program
pub(crate) mod runner;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// execute the brainfuck program
pub fn run(program: &str) -> String {
    let operation_codes = lex(program.to_string());

    let instructions = parse(operation_codes);

    let output = runner::run(&instructions);

    output
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn gives_correct_output() {
        let hello_world = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
        let output = run(hello_world);

        assert_eq!(output, "Hello World!\n")
    }
}
