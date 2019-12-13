pub type Word = i64;
pub type Label = String;
pub type Constant = String;
pub type Statements = Vec<Statement>;

#[derive(Debug,Eq,PartialEq)]
pub enum Statement {
    DirectiveStatement(Directive),
    InstructionStatement(Option<Label>, Instruction),
}

#[derive(Debug,Eq,PartialEq)]
pub struct Program {
    pub statements : Vec<Statement>,
}

#[derive(Debug,Eq,PartialEq)]
pub enum Directive {
    Define(Constant, Word),
    Foo,
}

#[derive(Debug,Eq,PartialEq)]
pub enum Instruction {
    Add {
        op1: ReadParameter,
        op2: ReadParameter,
        dst: WriteParameter,
    },
    Halt,
}


#[derive(Debug,Eq,PartialEq)]
pub enum ReadParameter {
    Imm(Word),
    Pos(Word),
    Rel(Word),
}

#[derive(Debug,Eq,PartialEq)]
pub enum WriteParameter {
    Pos(Word),
    Rel(Word),
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
