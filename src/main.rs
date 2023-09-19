use std::{env, fs::File, io::Read};

use lexer::lex;
use parser::parse;
use runner::run;

mod instructions;
mod lexer;
mod parser;
mod runner;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("usage: yabfi <any_file.bf>");
        std::process::exit(1);
    }

    let filename = &args[1];

    // read the file
    let mut file = File::open(filename).expect("that file does not exist");
    let mut source = String::new();
    file.read_to_string(&mut source)
        .expect("failed to read the file");

    // lex the file
    let operation_codes = lex(source);

    // parse opcodes
    let program = parse(operation_codes);

    // set up env and run
    let mut tape: Vec<u8> = vec![0; 1024];
    let mut data_ptr = 512;

    run(&program, &mut tape, &mut data_ptr)
}
