pub type Word = i64;
pub type Constant = String;
pub type Statements = Vec<Statement>;
pub type Label = String;

#[derive(Debug,Eq,PartialEq)]
pub enum Statement {
    DirectiveStatement(Directive),
    InstructionStatement(Option<Label>, Instruction),
    DeclarationStatement(Option<Label>, Instruction),
}

#[derive(Debug,Eq,PartialEq)]
pub struct Program {
    pub statements : Vec<Statement>,
}

#[derive(Debug,Eq,PartialEq)]
pub enum Directive {
    Def(Constant, Expression),
    Foo,
}

#[derive(Debug,Eq,PartialEq)]
pub enum Instruction {
    Add { op1: ReadParameter, op2: ReadParameter, dst: WriteParameter },
    Mul { op1: ReadParameter, op2: ReadParameter, dst: WriteParameter },
    In { dst: WriteParameter },
    Out { src: ReadParameter },
    Eq { op1: ReadParameter, op2: ReadParameter, dst: WriteParameter },
    Lt { op1: ReadParameter, op2: ReadParameter, dst: WriteParameter },
    Halt,
    Spa { val: Expression, }
}


#[derive(Debug,Eq,PartialEq)]
pub enum ReadParameter {
    ReadImm(Expression),
    ReadPos(Expression),
    ReadRel(Expression),
}

#[derive(Debug,Eq,PartialEq)]
pub enum WriteParameter {
    WritePos(Expression),
    WriteRel(Expression),
}

#[derive(Debug,Eq,PartialEq)]
pub enum Expression {
    Number(Word),
    Constant(Constant),
    Plus(Box<Expression>, Box<Expression>),
    Multiplication(Box<Expression>, Box<Expression>),
    Minus(Box<Expression>, Box<Expression>),
    UnaryMinus(Box<Expression>),
}



/*
#define count  100
#define comp_res 101
#define size 16
.loop   addrb 1                     ; comment, until end of line

.loop   addrb 1
        out [%rb-1]
        add [count] 1 [count]
        eq [tmp] size [comp]
        jif [comp] loop
        halt

>>.table  dw  1234,1235,4,127,88
>>.msg    dw  "hello world", 0

>>        jit
>>        add
>>        mul
>>        lt


>>statements = list of statements

>>statement = [.(<label>)] memonic

>>mnemonic = directive | data_expr | instruction_expr

>>data_expr = dw list of data, csv
>>data = <int> | "string"

  value = CONST|INT

#[test]
fn calculator() {
    assert!(calculator::TermParser::new().parse("22").is_ok());
    assert!(calculator::TermParser::new().parse("(22)").is_ok());
    assert!(calculator::TermParser::new().parse("((((22))))").is_ok());
    assert!(calculator::TermParser::new().parse("((22)").is_err());
}

  */
