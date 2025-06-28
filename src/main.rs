/*
SCREAM: The compiler for Chemical Head
*/

use std::env;

mod utils;
mod tools;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2{
        println!("Too many arguements, correct usage: SCREAM file.chem");
        std::process::exit(0);
    }

    println!("{}", utils::CHEMICAL_HEAD_LANG_ART);

    //let filepath: &str = &args[1];    FOR LATER
    let filepath: &str = "/Users/uglyprincess/Documents/Code/Rust/SCREAM/test.chem";

    tools::lexer::read_program(filepath);
}
