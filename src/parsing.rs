use crate::ast::Program;
use crate::ast;
use ast::{Statement, Expression};
use ast::Statement::DirectiveStatement;
use ast::Directive::Def;
use crate::lexer::{Lexer, LexicalError, Token};
use lalrpop_util::ParseError;

lalrpop_mod!(pub grammar); // syntesized by LALRPOP

pub type ParseProgramResult = Result<Program, ParseError<usize, Token, LexicalError>>;
pub type ParseStatementResult = Result<Statement, ParseError<usize, Token, LexicalError>>;

pub fn parse_statement(input: &str) -> ParseStatementResult {
    let parser = grammar::StmtParser::new();
    return parser.parse(Lexer::new(input))
}
pub fn parse_program(input: &str) -> ParseProgramResult {
    let parser = grammar::StmtsParser::new();
    return parser.parse(Lexer::new(input))
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use crate::ast::Directive::{Foo, Def};
    use crate::ast::Expression;
    use crate::ast::Declaration::{Dup, Dw};
    use crate::ast::Instruction::{Add, Jif, Jit};
    use crate::ast::Expression::{Number, Constant, Plus};
    use crate::ast::ReadParameter::{ReadPos, ReadImm};
    use crate::ast::WriteParameter::WriteRel;
    use crate::ast::Statement::{InstructionStatement, DeclarationStatement};
    use crate::ast::DataValue::{DataExpression, DataString};

    #[test]
    fn test_parse_foo() {
        let input = "#foo\n";
        let stmt = parse_statement(input).unwrap();
        assert_eq!(stmt, DirectiveStatement(Foo));
    }

    #[test]
    fn test_parse_define() {
        let input = "#def   bar 3\n";
        let stmt = parse_statement(input).unwrap();
        assert_eq!(stmt, DirectiveStatement(Def(String::from("bar"), Expression::Number(3))));
    }

    #[test]
    fn test_parse_define_expr() {
        let input = "#def  bar -1\n";
        let stmt = parse_statement(input).unwrap();
        assert_eq!(stmt, DirectiveStatement(
            Def(String::from("bar"),
                Expression::UnaryMinus(Box::new(Expression::Number(1))))));
    }

    #[test]
    fn test_parse_add() {
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

    #[test]
    fn test_parse_jif() {
        let input = ".l1     jif [1], l2 \n";
        let stmt = parse_statement(input).unwrap();
        assert_eq!(stmt, InstructionStatement(
            Some(String::from("l1")),
            Jif {
                cond: ReadPos(Number(1)),
                target: Constant(String::from("l2")),
            }));
    }
    #[test]
    fn test_parse_jit() {
        let input = ".l1     jit [1], l2 \n";
        let stmt = parse_statement(input).unwrap();
        assert_eq!(stmt, InstructionStatement(
            Some(String::from("l1")),
            Jit {
                cond: ReadPos(Number(1)),
                target: Constant(String::from("l2")),
            }));
    }
    #[test]
    fn test_parse_dw_single() {
        let input = ".l1     dw 0 \n";
        let stmt = parse_statement(input).unwrap();
        assert_eq!(stmt, DeclarationStatement(
            Some(String::from("l1")),
            Dw(vec![ DataExpression(Number(0)) ]),
        ));
    }
    #[test]
    fn test_parse_dw_string() {
        let input = ".l1     dw \"hello world!\"\n";
        let stmt = parse_statement(input).unwrap();
        assert_eq!(stmt, DeclarationStatement(
            Some(String::from("l1")),
            Dw(vec![ DataString(String::from("hello world!!")) ]),
        ));
    }
    #[test]
    fn test_parse_dw_multiple() {
        let input = ".l1     dw 0, 1, 2, 3 + 4 \n";
        let stmt = parse_statement(input).unwrap();
        assert_eq!(stmt, DeclarationStatement(
            Some(String::from("l1")),
            Dw(vec![
                DataExpression(Number(0)),
                DataExpression(Number(1)),
                DataExpression(Number(2)),
                DataExpression(Plus(Box::new(Number(3)), Box::new(Number(4))))
                ]),
        ));
    }

    #[test]
    fn test_parse_dup() {
        let input = ".l1     dup 3, 4 \n";
        let stmt = parse_statement(input).unwrap();
        assert_eq!(stmt, DeclarationStatement(
            Some(String::from("l1")),
            Dup(Number(3), Number(4))
            ),
        );
    }

}
