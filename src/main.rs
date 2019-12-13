#[macro_use]
extern crate lalrpop_util;

pub mod lexer;
pub mod ast;

use ast::{Constant, Statement};
use ast::Statement::DirectiveStatement;
use ast::Directive::Define;
use crate::lexer::{LexicalError, Token};
use lalrpop_util::ParseError;
use crate::ast::Program;

lalrpop_mod!(pub grammar); // syntesized by LALRPOP

fn main() {
    println!("Hello world");
}

pub type ParseResult = Result<Program, ParseError<usize, Token, LexicalError>>;

pub fn parse_string(input: &str) -> ParseResult {
    let parser = grammar::StmtsParser::new();
    parser.parse(lexer::Lexer::new(input))
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::{BufReader, Read};

    /*
    #[test]
    fn test_simple_directive() {
        let input = "#define something 3";
        let expr = parse_string(input).unwrap();
        assert_eq!(expr, DirectiveStatement(Define(String::from("something"), 3)));
    }

    #[test]
    fn test_foo() {
        let input = "#foo 3";
        let expr = parse_string(input).unwrap();
        assert_eq!(expr, DirectiveStatement(Define(String::from("something"), 3)));
    }
    */

    #[test]
    fn test_foo_file() {
        let filename = "examples/definitions.asm";
        let input  = std::fs::read_to_string(filename).unwrap();
        let expr = parse_string(&input).unwrap();
        assert_eq!(expr,
                   Program {
                       statements: vec![
                           DirectiveStatement(Define(String::from("something"), 3)),
                       ]
                   });
    }
}
