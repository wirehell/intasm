use std::str::CharIndices;
use crate::lexer::Token::{LineEnd, Define, Foo};
use std::iter::Peekable;
use crate::lexer::LexicalError::{UnexpectedEof, UnknownDirective, UnexpectedCharacter};
use std::ops::Deref;
use std::borrow::Borrow;

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


impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chars.next() {
                Some((i, '#')) => {
                    let mut directive = String::with_capacity(32);
                    let mut count = i;
                    while match self.chars.peek() {
                        Some((i, c)) if !c.is_whitespace() => {
                            directive.push(*c);
                            count = *i;
                            true
                        },
                        None => return Some(Err(UnexpectedEof)),
                        _ => false,
                    } {
                        self.chars.next();
                    }
                    return match directive.deref() {
                        "define" => Some(Ok((i, Define, count + 1))),
                        "foo" => Some(Ok((i, Foo, count + 1))),
                        x => Some(Err(UnknownDirective(String::from(x)))),
                    }
                },
                Some((i, '&')) => return Some(Err(LexicalError::Test)),
                Some((i, ';')) => {
                    while match self.chars.peek() {
                        Some((i, '\n')) => false,
                        None => false,
                        _ => true,
                    } {
                        self.chars.next();
                    }
                },
                Some((i, '\n')) | Some((i, '\r')) => {
                    return Some(Ok((i, LineEnd, i+1)));
                },
                None => return None, // End of file
                Some((i, c)) => {
                    println!("Panic: {}", c);
                    return Some(Err(UnexpectedCharacter));
                },
            };
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    WhiteSpace,
    LineEnd,
    Hash,
    Period,
    Comma,
    Quote,
    LBracket,
    RBracket,
    Percent,
    Define,
    Foo,
    EOF,
    String(String),
    Number(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LexicalError {
    UnknownDirective(String),
    UnexpectedCharacter,
    UnexpectedEof,
    Test
    // Not possible
}