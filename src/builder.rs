use std::rc::Rc;

use crate::calculator::Calculator;
use crate::reader::{Reader, EmptyInput};
use crate::lexer::{Lexer, EmptyLexer};
use crate::converters::{Converter, EmptyConverter};
use crate::writer::{Writer, EmptyOutput};
use crate::validator::Validator;

pub struct CalculatorBuilder {
    input: Rc<dyn Reader>,
    lexer: Rc<dyn Lexer>,
    validator: Rc<Validator>,
    converter: Rc<dyn Converter>,
    writer: Rc<dyn Writer>,
}

impl CalculatorBuilder {
    pub fn new() -> Self {
        Self {
            input: Rc::new(EmptyInput {}),
            lexer: Rc::new(EmptyLexer {}),
            validator: Rc::new(Validator::new()),
            converter: Rc::new(EmptyConverter {}),
            writer: Rc::new(EmptyOutput {}),
        }
    }

    pub fn input_stream<'a>(&'a mut self, input: Rc<dyn Reader>) -> &'a mut Self {
        self.input = input;
        self
    }

    pub fn lexer<'a>(&'a mut self, lex: Rc<dyn Lexer>) -> &'a mut Self {
        self.lexer = lex;
        self
    }

    pub fn validator<'a>(&'a mut self, v: Rc<Validator>) -> &'a mut Self {
        self.validator = v;
        self
    }

    pub fn converter<'a>(&'a mut self, conv: Rc<dyn Converter>) -> &'a mut Self {
        self.converter = conv;
        self
    }

    #[allow(dead_code)]
    pub fn output_stream<'a>(&'a mut self, writer: Rc<dyn Writer>) -> &'a mut Self {
        self.writer = writer;
        self
    }

    pub fn build(&self, hello: &'static str) -> Calculator {
        Calculator {
            hello_str: hello,
            input: self.input.clone(),
            lexer: self.lexer.clone(),
            validator: self.validator.clone(),
            converter: self.converter.clone(),
            writer: self.writer.clone(),
        }
    }
}
