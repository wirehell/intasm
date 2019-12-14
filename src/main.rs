#[macro_use]
extern crate lalrpop_util;

pub mod lexer;
pub mod ast;
pub mod parsing;
pub mod generation;

use crate::ast::Program;
use crate::parsing::parse_program;
use crate::generation::assemble;


fn main() {
    let filename = "example.asm";
    let input  = std::fs::read_to_string(filename).unwrap();

    let program = parse_program(&input);
    println!("Program is: {:?}", program);
    let output = assemble(&program.unwrap());

    println!("Output is: {:?}", output);
}


#[cfg(test)]
mod test {

}
