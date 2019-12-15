#[macro_use]
extern crate lalrpop_util;

pub mod lexer;
pub mod ast;
pub mod parsing;
pub mod generation;

use crate::ast::{Program, Word};
use crate::parsing::parse_program;
use crate::generation::assemble;
use std::fs::File;

use std::io::{Write, BufReader, BufRead, Error};


fn main() {
    let filename = "examples/gol.asm";
    let input  = std::fs::read_to_string(filename).unwrap();

    let program = parse_program(&input);
    println!("Program is: {:?}", program);
    let output = assemble(&program.unwrap());

    println!("Output is: {:?}", output);
    let i_vec :Vec<String> = output.iter().map(Word::to_string).collect();
    let joined :String = i_vec.join(",");
    println!("Output is: {:?}", joined);
    let mut file = File::create("i.out").unwrap();
    write!(file, "{}", joined);

}


#[cfg(test)]
mod test {

}
