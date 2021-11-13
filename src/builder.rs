use std::rc::Rc;

use crate::calculator::Calculator;
use crate::converters::Converter;
use crate::lexer::Lexer;
use crate::reader::Reader;
use crate::validator::Validator;
use crate::writer::Writer;

pub struct CalculatorBuilder {
    target: Calculator,
}

impl CalculatorBuilder {
    pub fn new() -> Self {
        Self {
            target: Calculator::new(),
        }
    }

    pub fn input_stream(&mut self, input: Rc<dyn Reader>) -> &mut Self {
        self.target.input = input;
        self
    }

    pub fn lexer(&mut self, lex: Rc<dyn Lexer>) -> &mut Self {
        self.target.lexer = lex;
        self
    }

    pub fn validator(&mut self, v: Rc<Validator>) -> &mut Self {
        self.target.validator = v;
        self
    }

    pub fn converter(&mut self, conv: Rc<dyn Converter>) -> &mut Self {
        self.target.converter = conv;
        self
    }

    #[allow(dead_code)]
    pub fn output_stream(&mut self, writer: Rc<dyn Writer>) -> &mut Self {
        self.target.writer = writer;
        self
    }

    pub fn build(&self, hello: &str) -> Calculator {
        Calculator {
            hello_str: hello.to_string(),
            input: self.target.input.clone(),
            lexer: self.target.lexer.clone(),
            validator: self.target.validator.clone(),
            converter: self.target.converter.clone(),
            writer: self.target.writer.clone(),
        }
    }
}
