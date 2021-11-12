use std::rc::Rc;

use crate::reader::{Reader, EmptyInput};
use crate::lexer::{Lexer, EmptyLexer};
use crate::converters::{Converter, EmptyConverter};
use crate::writer::{Writer, ConsoleOutput};
use crate::validator::Validator;
use crate::operator::{Expression, Number, Operator, Lexem};
use crate::stack::Stack;

pub struct Calculator {
    pub hello_str: String,
    pub input: Rc<dyn Reader>,
    pub lexer: Rc<dyn Lexer>,
    pub validator: Rc<Validator>,
    pub converter: Rc<dyn Converter>,
    pub writer: Rc<dyn Writer>,
}


impl Calculator {
    pub fn new() -> Self {
        Calculator {
            hello_str: String::from(""),
            input: Rc::new(EmptyInput {}),
            lexer: Rc::new(EmptyLexer {}),
            validator: Rc::new(Validator::new()),
            converter: Rc::new(EmptyConverter {}),
            writer: Rc::new(ConsoleOutput::default()),
        }
    }

    pub fn run(&mut self) {
        println!("{}", self.hello_str);
        let mut input_string = match self.input.read() {
            Ok(result) => result,
            Err(why) => {
                self.writer.print_error(format!("Ошибка получения входной строки: {}", why));
                return;
            },
        };

        let tokens = match self.lexer.tokenize(&mut input_string) {
            Ok(result) => result,
            Err(why) => {
                self.writer.print_error(
                    format!("Ошибка разбиения на лексемы:\n{0}\n {2:>1$} неизвестная лексема!",
                            input_string,
                            input_string.find(why).unwrap(),
                            "^"));
                return;
            },
        };

        let valid_tokens = match self.validator.validate(tokens) {
            Ok(result) => result,
            Err(why) => {
                self.writer.print_error(format!("Ошибка валидации: {}", why));
                return;
            },
        };

        let mut expr = match self.converter.convert(valid_tokens) {
            Ok(result) => result,
            Err(why) => {
                self.writer.print_error(format!("Ошибка преобразования: {}", why));
                return;
            },
        };

        let res = match self.calculate(&mut expr) {
            Ok(result) => result,
            Err(why) => {
                self.writer.print_error(format!("Ошибка вычисления : {}", why));
                return;
            },
        };

        self.writer.print_success(format!("\nРезультат выражения: {}", res));
    }

    pub fn calculate(&self, input: &mut Expression) -> Result<Number, &str> {
        let mut arguments_stack : Stack<Number> = Stack::new();

        while let Some(lexem) = input.dequeue() {
            match lexem {
                Lexem::NumberLex(v) => {
                    print!("{} ", v);
                    arguments_stack.push(v);
                },

                Lexem::OperatorLex(op) => {
                    match op {
                        Operator::Unary(op) => {
                            print!("{} ", op.name);
                            if let Some(arg) = arguments_stack.pop() {
                                let result = Some((op.apply)(arg));
                                arguments_stack.push(result.unwrap());
                                continue;
                            }

                            return Err("error ");
                        },
                        Operator::Binary(op) => {
                            print!("{} ", op.name);
                            if let Some(arg) = arguments_stack.pop() {
                                if let Some(arg2) = arguments_stack.pop() {
                                    let result = Some((op.apply)((arg, arg2)));
                                    arguments_stack.push(result.unwrap());
                                    continue;
                                }
                            }

                            return Err("error ");
                        },
                        _ => continue,
                    };
                },
            };
        }

        if let Some(result) = arguments_stack.pop() {
            if !arguments_stack.is_empty() {
                return Err("Очередь аргументов не пуста, но очередь операторов опустела");
            }

            return Ok(result)
        }

        Err("Не удалось вычислить выражение")
    }
}
