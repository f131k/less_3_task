//!

use std::cmp::Ordering;

use crate::token::{Token, TokenType};
use crate::queue::Queue;


pub type Number = f32;

pub enum Lexem {
    NumberLex(Number),
    OperatorLex(Operator),
}
pub type Expression = Queue<Lexem>;

impl Lexem {
    pub fn new(tok: &Token) -> Self {
        match tok.0 {
            TokenType::NumberInt | TokenType::NumberFloat => Lexem::NumberLex(
                tok.1.parse::<Number>().unwrap()
            ),
            TokenType::UnaryOperator | TokenType::BinaryOperator | TokenType::Function =>
                Lexem::OperatorLex(Operator::get_operator(tok)),
            _ => Lexem::NumberLex(0.0),
        }
    }
}

#[derive(Debug)]
pub struct BaseOperator<T> {
    pub name: String,
    priority: u32,
    is_left: bool,
    pub apply: fn(T) -> Number,
}

#[derive(Debug)]
pub enum Operator {
    Unary(BaseOperator<Number>),
    Binary(BaseOperator<(Number, Number)>),
    #[allow(dead_code)]
    BinaryFunction(BaseOperator<(Number, Number)>),
    Unknown,
}

impl BaseOperator<Number> {
    fn new(
        n: String,
        p: u32,
        l: bool,
        f: fn(Number) -> Number
    ) -> Operator {
        Operator::Unary(
            BaseOperator::<Number> {
                name: n,
                priority: p,
                is_left: l,
                apply: f,
            }
        )
    }
}

impl BaseOperator<(Number, Number)> {
    fn new(
        n: String,
        p: u32,
        l: bool,
        f: fn((Number, Number)) -> Number
    ) -> Operator {
        Operator::Binary(
            BaseOperator::<(Number, Number)> {
                name: n,
                priority: p,
                is_left: l,
                apply: f,
            }
        )
    }
}

impl PartialOrd for Operator {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        println!("Сравниваем операторы {:?} {:?}",
                 self,
                 other);
        let (p_lhs, _) = match self {
            Operator::Unary(op) => (op.priority, op.is_left),
            Operator::Binary(op) => (op.priority, op.is_left),
            _ => return None,
        };

        let (p_rhs, _) = match other {
            Operator::Binary(op) => (op.priority, op.is_left),
            Operator::Unary(op) => (op.priority, op.is_left),
            _ => return None,
        };

        p_lhs.partial_cmp(&p_rhs)
    }
}

impl PartialEq for Operator {
    fn eq(&self, other: &Self) -> bool {
        println!("Сравниваем операторы {:?} {:?}",
                 self,
                 other);
        let (p_lhs, l_lhs) = match self {
            Operator::Unary(op) => (op.priority, op.is_left),
            Operator::Binary(op) => (op.priority, op.is_left),
            _ => return false,
        };

        let (p_rhs, _) = match other {
            Operator::Binary(op) => (op.priority, op.is_left),
            Operator::Unary(op) => (op.priority, op.is_left),
            _ => return false,
        };

        if l_lhs {
            println!("Оператор левоассоциативный. Приоритеты {} == {}", p_lhs, p_rhs);
            p_lhs == p_rhs
        } else {
            false
        }
    }
}

impl Operator {
    pub fn get_operator(tok: &Token) -> Operator {
        match tok.1.as_ref() {
            "+" if tok.0 == TokenType::UnaryOperator => BaseOperator::<Number>::new("POS".to_string(), 1, true, |x| x),
            "-" if tok.0 == TokenType::UnaryOperator => BaseOperator::<Number>::new("NEG".to_string(), 1, true, |x| -1.0 * x),

            ">>" => BaseOperator::<(Number, Number)>::new("".to_string(), 4, true, |(x, y)| ((x as i32) >> (y as i32)) as Number),
            "<<" => BaseOperator::<(Number, Number)>::new("".to_string(), 4, true, |(x, y)| ((x as i32) << (y as i32)) as Number),

            "+" => BaseOperator::<(Number, Number)>::new("+".to_string(), 3, true, |(x, y)| x + y),
            "-" => BaseOperator::<(Number, Number)>::new("-".to_string(), 3, true, |(x, y)| x - y),

            "/" => BaseOperator::<(Number, Number)>::new("/".to_string(), 2, true, |(x, y)| x / y),
            "*" => BaseOperator::<(Number, Number)>::new("×".to_string(), 2, true, |(x, y)| x * y),
            "%" => BaseOperator::<(Number, Number)>::new("%".to_string(), 2, true, |(x, y)| x % y),

            "^" => BaseOperator::<(Number, Number)>::new("pow".to_string(), 5, false, |(x, y)| x.powf(y)),

            _ => Operator::Unknown,
        }
    }
}
