use crate::ast;
use crate::ast::{Instruction, Statement, Word, Declaration, Program, Directive, Expression, DataValue, ReadParameter, WriteParameter};
use std::collections::{HashMap, VecDeque};
use std::borrow::Borrow;

pub fn assemble(program :&Program) -> Vec<Word> {
    // Resolve constants
    let mut env = Env::new();
    resolve_constants(&mut env, &program);

    // First pass to determine labels
    let size = update_labels(&mut env, &program);
    let output = generate_machine_code(&env, &program);

    assert_eq!(size, output.len() as i64);
    return output;
}

fn generate_machine_code(env :&Env, program: &Program) -> Vec<Word> {

    let mut machine_code = vec![];
    for statement in &program.statements {
        match statement {
            Statement::DirectiveStatement(_) => {},
            Statement::InstructionStatement(_, instruction) => {
                machine_code.extend(instruction.encode(&env));
            },
            Statement::DeclarationStatement(_, declaration) => {
                machine_code.extend(declaration.encode(&env));
            },
        }

    }
    return machine_code;
}

struct Env {
   symbols: HashMap<String, Word>,
}

impl Env {
    pub fn new() -> Env {
        Env { symbols: HashMap::new() }
    }

    pub fn add_constant(&mut self, c :&str, val: Word) {
        println!("Adding constant: {} = {}", &c, &val);
        let status = self.symbols.insert(String::from(c), val);
        if status.is_some() {
            panic!("Already defined: {}", c);
        }
    }

    pub fn add_label(&mut self, l :&str, address: Word) {
        println!("Adding label: {} = {}", &l, &address);
        let status = self.symbols.insert(String::from(l), address);
        if status.is_some() {
            panic!("Already defined: {}", l);
        }

    }
}


fn update_labels(env :&mut Env, program: &Program) -> Word {
    let mut current_address :Word = 0;
    for statement in &program.statements {
        match statement {
            Statement::DirectiveStatement(_) => {},
            Statement::InstructionStatement(m_l, i) => {
                match m_l {
                    None => {},
                    Some(label) => {
                        env.add_label(label, current_address)
                    },
                }
                current_address += i.size();

            },
            Statement::DeclarationStatement(m_l, d) => {
                match m_l {
                    None => {},
                    Some(label) => {
                        env.add_label(label, current_address)
                    },
                }
                current_address += d.size(&env);
            },
        }
    }
    return current_address;
}


fn resolve_constants(env: &mut Env, program :&Program) {
    for statement in &program.statements {
        match statement {
            Statement::DirectiveStatement(ds) => {
                match ds {
                    Directive::Def(c, e) => {
                        let val = e.evaluate(&env);
                        env.add_constant(c, val);
                    },
                    Directive::Foo => {},
                }
            },
            Statement::InstructionStatement(_, _) => {},
            Statement::DeclarationStatement(_, _) => {},
        }
    }
}


impl Expression {
    fn evaluate(&self, env: &Env) -> Word {
        return match self {
            Expression::Number(n) => *n,
            Expression::Constant(c) => {
                match env.symbols.get(c) {
                    Some(value) => *value,
                    None => panic!("Unknown symbol: {}", c),
                }
            }
            Expression::Plus(a, b) =>
                a.evaluate(&env) + b.evaluate(&env),
            Expression::Multiplication(a, b) =>
                a.evaluate(&env) * b.evaluate(&env),
            Expression::Minus(a, b) =>
                a.evaluate(&env) - b.evaluate(&env),
            Expression::UnaryMinus(a) => -a.evaluate(&env),
        };
    }
}

impl Statement {
    fn size(&self, env: &Env) -> Word {
        match &self {
            Statement::DirectiveStatement(_) => 0,
            Statement::InstructionStatement(_, i) => i.size(),
            Statement::DeclarationStatement(_, d) => d.size(&env),
        }
    }
}

impl Instruction {
    fn size(&self) -> Word {
        match self {
            Instruction::Add { .. } => 4,
            Instruction::Mul { .. } => 4,
            Instruction::In { .. } => 2,
            Instruction::Out { .. } => 2,
            Instruction::Eq { .. } => 4,
            Instruction::Lt { .. } => 4,
            Instruction::Halt => 1,
            Instruction::Spa { .. } => 2,
            Instruction::Jit { .. } => 3,
            Instruction::Jif { .. } => 3,
        }
    }

    fn encode(&self, env: &Env) -> Vec<Word> {
        return match self {
            Instruction::Add { op1, op2, dst } => {
                let p_mode = op1.get_mode(0) + op2.get_mode(1) + dst.get_mode(2);
                vec![1 + p_mode, op1.get_value(env), op2.get_value(env), dst.get_value(env)]
            },
            Instruction::Mul { op1, op2, dst } => {
                let p_mode = op1.get_mode(0) + op2.get_mode(1) + dst.get_mode(2);
                vec![2 + p_mode, op1.get_value(env), op2.get_value(env), dst.get_value(env)]
            },
            Instruction::In { dst } => {
                let p_mode = dst.get_mode(0);
                vec![3 + p_mode, dst.get_value(env)]
            },
            Instruction::Out { src } => {
                let p_mode = src.get_mode(0);
                vec![4 + p_mode, src.get_value(env)]
            },
            Instruction::Jit { cond, target } => {
                let p_mode = cond.get_mode(0) + target.get_mode(1);
                vec![5 + p_mode, cond.get_value(env), target.get_value(env)]
            },
            Instruction::Jif { cond, target } => {
                let p_mode = cond.get_mode(0) + target.get_mode(1);
                vec![6 + p_mode, cond.get_value(env), target.get_value(env)]
            },
            Instruction::Lt { op1, op2, dst } => {
                let p_mode = op1.get_mode(0) + op2.get_mode(1) + dst.get_mode(2);
                vec![7 + p_mode, op1.get_value(env), op2.get_value(env), dst.get_value(env)]
            },
            Instruction::Eq { op1, op2, dst } => {
                let p_mode = op1.get_mode(0) + op2.get_mode(1) + dst.get_mode(2);
                vec![8 + p_mode, op1.get_value(env), op2.get_value(env), dst.get_value(env)]
            },
            Instruction::Spa { val } => {
                let p_mode = val.get_mode(0);
                vec![9 + p_mode, val.get_value(env)]
            },
            Instruction::Halt => {
                vec![99]
            },
            _ => unreachable!(),
        };
    }
}

trait ParameterMode {
    fn get_mode(&self, pos:u32) -> Word;
    fn get_value(&self, env: &Env) -> Word;
}

impl ParameterMode for ReadParameter {
    fn get_mode(&self, pos: u32) -> i64 {
        let exp:i64 = 10;
        let m = match self {
            ReadParameter::ReadPos(v) => 0,
            ReadParameter::ReadImm(v) => 1,
            ReadParameter::ReadRel(v) => 2,
        };
        return m * exp.pow(pos+2);
    }

    fn get_value(&self, env: &Env) -> Word {
        return match self {
            ReadParameter::ReadImm(v) => v.evaluate(env),
            ReadParameter::ReadPos(v) => v.evaluate(env),
            ReadParameter::ReadRel(v) => v.evaluate(env),
        };
    }
}

impl ParameterMode for WriteParameter {
    fn get_mode(&self, pos: u32) -> i64 {
        let exp:i64 = 10;
        let m = match self {
            WriteParameter::WritePos(v) => 0,
            WriteParameter::WriteRel(v) => 2,
        };
        return m * exp.pow(pos+2);
    }

    fn get_value(&self, env: &Env) -> i64 {
        return match self {
            WriteParameter::WritePos(v) => v.evaluate(env),
            WriteParameter::WriteRel(v) => v.evaluate(env),
        };
    }
}



impl Declaration {
    fn size(&self, env: &Env) -> Word {
        match self {
            Declaration::Dw(v) => {
                let mut c = 0;
                return v.iter().map(DataValue::size).sum();
            },
            Declaration::Dup(c, e) => {
                return c.evaluate(env);
            },
        }
    }

    fn encode(&self, env: &Env) -> Vec<Word> {
        match self {
            Declaration::Dw(v) => {
                let mut res = vec![];
                for value in v {
                    res.extend(value.encode(env));
                }
                return res;
            },
            Declaration::Dup(c, v) => {
                return vec![v.evaluate(env) ;c.evaluate(env) as usize]
            },
        }
    }

}

impl DataValue {
    fn size(&self) -> Word {
        match self {
            DataValue::DataExpression(_) => 1,
            DataValue::DataString(s) => s.len() as Word,
        }
    }

    fn encode(&self, env: &Env) -> Vec<Word> {
        match self {
            DataValue::DataExpression(e) => {
                return vec![e.evaluate(env)];
            },
            DataValue::DataString(s) => {
                let res:Vec<Word>= s.chars().map(|x| x as Word).collect();
                return res;
            },
        }
    }
}

