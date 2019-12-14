pub type Word = i64;
pub type Constant = String;
pub type Statements = Vec<Statement>;
pub type Label = String;

#[derive(Debug,Eq,PartialEq)]
pub struct Program {
    pub statements : Vec<Statement>,
}

#[derive(Debug,Eq,PartialEq)]
pub enum Statement {
    DirectiveStatement(Directive),
    InstructionStatement(Option<Label>, Instruction),
    DeclarationStatement(Option<Label>, Declaration),
}

#[derive(Debug,Eq,PartialEq)]
pub enum Directive {
    Def(Constant, Expression),
    Foo,
}

#[derive(Debug,Eq,PartialEq)]
pub enum DataValue {
    DataExpression(Expression),
    DataString(String),
}

#[derive(Debug,Eq,PartialEq)]
pub enum Declaration {
    Dw(Vec<DataValue>),
    Dup(Expression, Expression),
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
    Spa { val: Expression, },
    Jit { cond: ReadParameter, target: Expression },
    Jif { cond: ReadParameter, target: Expression },
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

