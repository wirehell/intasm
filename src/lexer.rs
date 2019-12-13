use std::str::{CharIndices, FromStr};
use std::iter::Peekable;
use crate::lexer::LexicalError::{UnexpectedEof, UnknownDirective, UnexpectedCharacter};
use std::ops::Deref;
use std::borrow::Borrow;
use crate::ast::Word;

pub type Spanned<Token, Loc, LexicalError> = Result<(Loc, Token, Loc), LexicalError>;

pub struct Lexer<'input> {
    chars: Peekable<CharIndices<'input>>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            chars: input.char_indices().peekable(),
        }
    }
}

// Keywords, strings and integers
fn string_to_token(s :String) -> Token {
    return match s.borrow() {
        "def"       => Token::Def,
        "foo"       => Token::Foo,
        "add"       => Token::Add,
        "mul"       => Token::Mul,
        "in"        => Token::In,
        "out"       => Token::Out,
        "jit"       => Token::Jit,
        "jif"       => Token::Jif,
        "lt"        => Token::Lt,
        "eq"        => Token::Eq,
        "halt"      => Token::Halt,
        "spa"       => Token::Spa,
        "dw"        => Token::DW,
        "dup"       => Token::Dup,
        alphanumeric => {
            match Word::from_str(&alphanumeric) {
                Ok(val) => Token::NumberLiteral(val),
                Err(e) => Token::StringLiteral(s),
            }
        },
    }
}

fn allowed_chars(c :&char) -> bool {
    return match c {
        s if s.is_alphanumeric() => true,
        '_' => true,
        _ => false,
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chars.next() {
                Some((i, '#')) => {
                    return Some(Ok((i, Token::Hash, i + 1)))
                },
                Some((i, '+')) => {
                    return Some(Ok((i, Token::Plus, i + 1)))
                },
                Some((i, '-')) => {
                    return Some(Ok((i, Token::Minus, i + 1)))
                },
                Some((i, '.')) => {
                    return Some(Ok((i, Token::Period, i + 1)))
                },
                Some((i, ',')) => {
                    return Some(Ok((i, Token::Comma, i + 1)))
                },
                Some((i, '%')) => {
                    return Some(Ok((i, Token::Percent, i + 1)))
                },
                Some((i, '[')) => {
                    return Some(Ok((i, Token::LBracket, i+1)));
                },
                Some((i, ']')) => {
                    return Some(Ok((i, Token::RBracket, i+1)));
                },
                Some((i, '(')) => {
                    return Some(Ok((i, Token::LParen, i+1)));
                },
                Some((i, ')')) => {
                    return Some(Ok((i, Token::RParen, i+1)));
                },
                Some((i, '*')) => {
                    return Some(Ok((i, Token::Asterix, i+1)));
                },
                Some((i, '\n')) | Some((i, '\r')) => {
                    return Some(Ok((i, Token::LineEnd, i+1)));
                },
                Some((i, ' ')) | Some((i, '\t')) => {
                    while {
                        match self.chars.peek() {
                            Some((_, ' ')) | Some((_, '\t')) => true,
                            Some(_) => false,
                            None => false,
                        }
                    } {
                        self.chars.next();
                    }
                },
                Some((i, c)) if allowed_chars(&c) => {
                    let mut s = String::with_capacity(32);
                    let mut current = c;
                    loop {
                        s.push(current);
                        let next = self.chars.peek();
                        match next {
                            Some((i, c)) if allowed_chars(c) => {}
                            _ => break,
                        }
                        current = self.chars.next().unwrap().1; // Just checked it, so unwrap
                    }

                    let next = s.len() + 1;
                    println!("Found: |{}|", &s);
                    let token = string_to_token(s);
                    return Some(Ok((i, token, next)));
                },
                Some((i, ';')) => {
                    while match self.chars.peek() {
                        Some((i, '\n')) => false,
                        None => false,
                        _ => true,
                    } {
                        self.chars.next();
                    }
                },
                Some((i, c)) => {
                    return Some(Err(UnexpectedCharacter));
                },
                None => return None, // End of file
            };
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    LineEnd,
    Hash,
    Period,
    Comma,
    Quote,
    Plus,
    Minus,
    Asterix,
    LBracket,
    RBracket,
    LParen,
    RParen,
    Percent,
    Def,
    Foo,
    EOF,
    StringLiteral(String),
    NumberLiteral(Word),
    Add,
    Mul,
    In,
    Out,
    Jit,
    Jif,
    Lt,
    Eq,
    Halt,
    Spa,
    DW,
    Dup,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LexicalError {
    UnknownDirective(String),
    UnexpectedCharacter,
    UnexpectedEof,
}