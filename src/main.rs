#[macro_use]
extern crate lalrpop_util;

pub mod lexer;
pub mod ast;

use ast::{Constant, Statement};
use ast::Statement::DirectiveStatement;
use ast::Directive::Def;
use crate::lexer::{LexicalError, Token};
use lalrpop_util::ParseError;
use crate::ast::Program;

lalrpop_mod!(pub grammar); // syntesized by LALRPOP

fn main() {
    println!("Hello world");
}

pub type ParseProgramResult = Result<Program, ParseError<usize, Token, LexicalError>>;
pub type ParseStatementResult = Result<Statement, ParseError<usize, Token, LexicalError>>;

pub fn parse_statement(input: &str) -> ParseStatementResult {
    let parser = grammar::StmtParser::new();
    return parser.parse(lexer::Lexer::new(input))
}
pub fn parse_program(input: &str) -> ParseProgramResult {
    let parser = grammar::StmtsParser::new();
    return parser.parse(lexer::Lexer::new(input))
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::{BufReader, Read};
    use crate::ast::Directive::{Foo, Def};
    use crate::ast::Expression;
    use crate::ast::Instruction::Add;
    use crate::ast::Expression::Number;
    use crate::ast::ReadParameter::{ReadPos, ReadImm};
    use crate::ast::WriteParameter::WriteRel;
    use crate::ast::Statement::InstructionStatement;

    #[test]
    fn test_foo() {
        let input = "#foo\n";
        let stmt = parse_statement(input).unwrap();
        assert_eq!(stmt, DirectiveStatement(Foo));
    }

    #[test]
    fn test_define() {
        let input = "#def   bar 3\n";
        let stmt = parse_statement(input).unwrap();
        assert_eq!(stmt, DirectiveStatement(Def(String::from("bar"), Expression::Number(3))));
    }

    #[test]
    fn test_define_expr() {
        let input = "#def  bar -1\n";
        let stmt = parse_statement(input).unwrap();
        assert_eq!(stmt, DirectiveStatement(
            Def(String::from("bar"),
                Expression::UnaryMinus(Box::new(Expression::Number(1))))));
    }

    #[test]
    fn test_add() {
        let input = ".label     add 1, [2], %[3]\n";
        let stmt = parse_statement(input).unwrap();
        assert_eq!(stmt, InstructionStatement(
            Some(String::from("label")),
            Add {
                op1: ReadImm(Number(1)),
                op2: ReadPos(Number(2)),
                dst: WriteRel(Number(3))
            }));
    }

    /*
    #[test]
    fn test_directives() {
        let filename = "examples/directives.asm";
        let input  = std::fs::read_to_string(filename).unwrap();
        let expr = parse_program(&input).unwrap();
        assert_eq!(expr,
                   Program {
                       statements: vec![
                           DirectiveStatement(Foo),
                           DirectiveStatement(Define(String::from("test"), 3)),
                           DirectiveStatement(Foo),
                           DirectiveStatement(Define(String::from("hest"), -4)),
                       ]
                   });
    }

    #[test]
    fn test_instructions() {
        let filename = "examples/instructions.asm";
        let input  = std::fs::read_to_string(filename).unwrap();
        let expr = parse_program(&input).unwrap();
        for statement in expr.statements {
            println!("{:?}", &statement);
        }
        /*
        assert_eq!(expr,
                   Program {
                       statements: vec![
                           DirectiveStatement(Foo),
                           DirectiveStatement(Define(String::from("test"), 3)),
                           DirectiveStatement(Foo),
                           DirectiveStatement(Define(String::from("hest"), -4)),
                       ]
                   });
                   */
    }
    */
}
