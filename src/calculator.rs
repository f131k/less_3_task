use std::rc::Rc;

use crate::reader::{Reader};
use crate::lexer::{Lexer};
use crate::converters::{Converter};
use crate::writer::{Writer};
use crate::validator::Validator;

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
                println!("Ошибка разбиения на лексемы: {}", why);
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

        let expr = match self.converter.convert(valid_tokens) {
            Ok(result) => result,
            Err(why) => {
                println!("Ошибка преобразования: {}", why);
                return;
            },
        };
    }
}
