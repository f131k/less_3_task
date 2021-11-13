use std::io;
use std::rc::Rc;

mod builder;
mod calculator;
mod converters;
mod lexer;
mod operator;
mod queue;
mod reader;
mod stack;
mod token;
mod validator;
mod writer;

use crate::builder::CalculatorBuilder;
use crate::converters::InfixToRPN;
use crate::lexer::RegexpLexer;
use crate::reader::ConsoleReader;
use crate::token::{Token, TokenList, TokenType};
use crate::validator::Validator;

pub type Rule = fn(&TokenList) -> bool;

///
/// Вывод приветственного сообщения на стандартный вывод
///
fn print_help() {
    println!(
        r#"Данная программа преобразует арифметическую операцию записанную в инфиксной форме в запись обратной польской нотации и вычисляет её.
Поддерживаемые операции:
  унарные:
    '+'
    '-'
  бинарные:
    '+'
    '-'
    '/'
    '*'
Для выхода нажмите <Ctrl+C>"#
    );
}

///
/// Запрос, выводимый на стандартный вывод нужно ли продолжать работу
///
fn request_to_continue() -> bool {
    let mut answer = String::new();
    println!("Продолжить (Д/н)");
    io::stdin()
        .read_line(&mut answer)
        .expect("Не удалось прочитать строку");
    match answer.trim() {
        "y" | "Y" | "Д" | "д" => return true,
        "n" | "N" | "Н" | "н" => return false,
        _ => {
            println!("Некорректный ввод. Закрываемся..");
        }
    }

    false
}


///
/// Вспомогательные функции передаваемые объекту валидатору для проверки введенных токенов
/// При первом разборе на токены все операторы сложения и вычитания по-умолчанию считаются
///  унарными. После успешного разбора на токены проходим по их списку с целью поиска реальных
///  унарных операторов и заменяем на бинарные где это необходимо.
/// Бинарность определяется по наличию перед проверяемым токеном закрывающей скобки или числа
///
fn check_for_binary_operator(list: &mut TokenList) -> bool {
    let mut ind: usize = 0;
    let list_size: usize = list.len();

    while ind < list_size {
        let tok: &Token = list.get(ind).unwrap();
        if tok.0 == TokenType::UnaryOperator {
            if ind > 0 {
                let prev: &Token = list.get(ind - 1).unwrap();
                if prev.0 == TokenType::NumberInt
                    || prev.0 == TokenType::NumberFloat
                    || prev.0 == TokenType::ClosedParenthesis
                {
                    list.get_mut(ind).unwrap().0 = TokenType::BinaryOperator;
                }
            }
        }
        ind += 1;
    }

    true
}

///
/// Вспомогательные функции передаваемые объекту валидатору для проверки введенных токенов
/// Проверка на наличие двух следующих подряд бинарных операторов
///
fn check_for_repeate_binary_operator(list: &mut TokenList) -> bool {
    let mut ind: usize = 0;
    let list_size: usize = list.len();

    while ind < list_size {
        let tok: &Token = list.get(ind).unwrap();
        if tok.0 == TokenType::BinaryOperator {
            if ind > 0 {
                let prev: &Token = list.get(ind - 1).unwrap();
                if prev.0 == TokenType::BinaryOperator {
                    return false;
                }
            }
        }
        ind += 1;
    }

    true
}


///
/// Точка входа
///
fn main() {
    print_help();

    // Создаем объект валидатора и добавляем требуемые правила-проверки
    let mut validator = Validator::new();
    validator
        .add_rule(check_for_binary_operator)
        .add_rule(check_for_repeate_binary_operator);

    // конструируем объект калькулятора, устанавливая необходимые конкретные
    // имплементации требуемых для вычисления объектов
    let mut calc = CalculatorBuilder::new()
        .input_stream(Rc::new(ConsoleReader {}))
        .lexer(Rc::new(RegexpLexer::new()))
        .validator(Rc::from(validator))
        .converter(Rc::new(InfixToRPN {}))
        .build("");

    // основной цикл
    loop {
        calc.run();

        match request_to_continue() {
            true => continue,
            false => break,
        }
    }
}
