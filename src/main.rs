use std::io;
use std::rc::Rc;

mod operator;
mod token;
mod reader;
mod lexer;
mod converters;
mod writer;
mod builder;
mod calculator;
mod validator;
mod stack;
mod queue;

use crate::builder::CalculatorBuilder;
use crate::reader::{ConsoleReader};
use crate::lexer::{RegexpLexer};
use crate::converters::{InfixToRPN};
use crate::validator::Validator;
use crate::token::{Token, TokenType, TokenList};

pub type Rule = fn(&TokenList) -> bool;

fn print_help() {
    println!(r#"Данная программа преобразует арифметическую операцию записанную в инфиксной форме в запись обратной польской нотации и вычисляет её.
Поддерживаемые операции:
  унарные:
    '+'
    '-'
  бинарные:
    '+'
    '-'
    '/'
    '*'
Для выхода нажмите <Ctrl+C>"#);
}

fn request_to_continue() -> bool {
    let mut answer = String::new();
    println!("Продолжить (Д/н)");
    io::stdin().read_line(&mut answer).expect("Не удалось прочитать строку");
    match answer.trim() {
        "y" | "Y" | "Д" | "д" => {return true},
        "n" | "N" | "Н" | "н" => {return false},
        _ => {
            println!("Некорректный ввод. Закрываемся..");
        },
    }

    false
}

fn check_for_binary_operator(list: &mut TokenList) -> bool {
    let mut ind : usize = 0;
    let list_size : usize = list.len();

    while ind < list_size {
        let tok : &Token  = list.get(ind).unwrap();
        if tok.0 == TokenType::UnaryOperator {
            if ind > 0 {
                let prev : &Token = list.get(ind - 1).unwrap();
                if prev.0 == TokenType::NumberInt ||
                    prev.0 == TokenType::NumberFloat ||
                    prev.0 == TokenType::ClosedParenthesis {
                        list.get_mut(ind).unwrap().0 =  TokenType::BinaryOperator;
                    }
            }
        }
        ind += 1;
    }

    true
}

fn check_for_repeate_binary_operator(list: &mut TokenList) -> bool {
    let mut ind : usize = 0;
    let list_size : usize = list.len();

    while ind < list_size {
        let tok : &Token  = list.get(ind).unwrap();
        if tok.0 == TokenType::BinaryOperator {
            if ind > 0 {
                let prev : &Token = list.get(ind - 1).unwrap();
                if prev.0 == TokenType::BinaryOperator {
                    return false
                }
            }
        }
        ind += 1;
    }

    true
}

fn main() {
    print_help();

    let mut validator = Validator::new();
    validator.add_rule(check_for_binary_operator)
        .add_rule(check_for_repeate_binary_operator);

    let mut calc = CalculatorBuilder::new()
        .input_stream(Rc::new(ConsoleReader {}))
        .lexer(Rc::new(RegexpLexer::new()))
        .validator(Rc::from(validator))
        .converter(Rc::new(InfixToRPN {}))
        .build("");

    loop {
        calc.run();

        match request_to_continue() {
            true => continue,
            false => break,
        }
    }
}
