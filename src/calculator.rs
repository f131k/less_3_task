use std::rc::Rc;

extern crate termion;

use termion::{color, style};

use crate::reader::{Reader};
use crate::lexer::{Lexer};
use crate::converters::{Converter};
use crate::writer::{Writer};
use crate::validator::Validator;
use crate::operator::{Expression, Number, Operator};

pub struct Calculator {
    pub hello_str: &'static str,
    pub input: Rc<dyn Reader>,
    pub lexer: Rc<dyn Lexer>,
    pub validator: Rc<Validator>,
    pub converter: Rc<dyn Converter>,
    pub writer: Rc<dyn Writer>,
}


impl Calculator {
    pub fn run(&mut self) {
        println!("{}", self.hello_str);
        let mut input_string = match self.input.read() {
            Ok(result) => result,
            Err(why) => {
                println!("Ошибка получения входной строки: {}", why);
                return;
            },
        };

        let tokens = match self.lexer.tokenize(&mut input_string) {
            Ok(result) => result,
            Err(why) => {
                println!("Ошибка разбиения на лексемы:\n{0}\n {2:>1$} {3}неизвестная лексема!{4}",
                         input_string,
                         input_string.find(why).unwrap(),
                         "^",
                         color::Fg(color::Red),
                         style::Reset);
                return;
            },
        };

        let valid_tokens = match self.validator.validate(tokens) {
            Ok(result) => result,
            Err(why) => {
                println!("Ошибка валидации: {}", why);
                return;
            },
        };

        let mut expr = match self.converter.convert(valid_tokens) {
            Ok(result) => result,
            Err(why) => {
                println!("Ошибка преобразования: {}", why);
                return;
            },
        };

        let res = match self.calculate(&mut expr) {
            Ok(result) => result,
            Err(why) => {
                println!("Ошибка вычисления : {}", why);
                return;
            },
        };

        println!("\nРезультат выражения: {}", res);
    }

    pub fn calculate(&self, input: &mut Expression) -> Result<Number, &str> {
        let args = &mut input.0;
        let ops = &mut input.1;
        let mut result : Option<Number> = None;

        while !ops.is_empty() {
            let op = ops.dequeue().unwrap();
            match op {
                Operator::Unary(op) => {
                    println!("{:?}", op);
                    if let Some(arg) = result {
                        result = Some((op.apply)(arg));
                        continue;
                    } else if let Some(arg) = args.dequeue() {
                        result = Some((op.apply)(arg));
                        continue;
                    }

                    return Err("error ");
                },
                Operator::Binary(op) => {
                    println!("{:?}", op);
                    if let Some(arg) = result {
                        if let Some(arg2) = args.dequeue() {
                            result = Some((op.apply)((arg, arg2)));
                            continue;
                        }
                    } else {
                        if let Some(arg) = args.dequeue() {
                            if let Some(arg2) = args.dequeue() {
                                result = Some((op.apply)((arg, arg2)));
                                continue;
                            }
                        }
                    }

                    return Err("error ");
                },
                _ => (),
            };
        }

        if !args.is_empty() {
            Err("Очередь аргументов не пуста, но очередь операторов опустела")
        } else {
            Ok(result.unwrap())
        }
    }
}
