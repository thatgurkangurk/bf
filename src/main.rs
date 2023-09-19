use args::parse_args;
use file::read_file_contents;
use lexer::lex;
use parser::parse;
use runner::run;

mod args;
mod file;
mod instructions;
mod lexer;
mod parser;
mod runner;

fn main() {
    let args = parse_args();
    let file = read_file_contents(&args.file);
    // lex the file
    let operation_codes = lex(file);

    // parse opcodes
    let program = parse(operation_codes);

    run(&program);
}
