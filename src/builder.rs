use std::rc::Rc;

use crate::calculator::Calculator;
use crate::converters::Converter;
use crate::lexer::Lexer;
use crate::reader::Reader;
use crate::validator::Validator;
use crate::writer::Writer;

///
/// Строитель калькулятора
/// Используется для получения объекта калькулятора с заданными настройками
///
pub struct CalculatorBuilder {
    target: Calculator,
}

// Реализация методов строителя
impl CalculatorBuilder {
    // создание нового объекта
    pub fn new() -> Self {
        Self {
            target: Calculator::new(),
        }
    }

    ///
    /// Установка объекта получаещего входную строку. Должен реализовывать типаж Readed
    ///
    pub fn input_stream(&mut self, input: Rc<dyn Reader>) -> &mut Self {
        self.target.input = input;
        self
    }

    ///
    /// Установка объекта разбирающего входную строку на токены. Должен реализовывать типаж Lexer
    ///
    pub fn lexer(&mut self, lex: Rc<dyn Lexer>) -> &mut Self {
        self.target.lexer = lex;
        self
    }

    ///
    /// Установка объекта проверяющего список токенов. Должен реализовывать типаж Validator
    ///
    pub fn validator(&mut self, v: Rc<Validator>) -> &mut Self {
        self.target.validator = v;
        self
    }

    ///
    /// Установка объекта преобразующих список токенов в выражение для вычисления. Должен реализовывать типаж Converter
    ///
    pub fn converter(&mut self, conv: Rc<dyn Converter>) -> &mut Self {
        self.target.converter = conv;
        self
    }

    ///
    /// Установка объекта выводящего данные. Должен реализовывать типаж Writer
    ///
    #[allow(dead_code)]
    pub fn output_stream(&mut self, writer: Rc<dyn Writer>) -> &mut Self {
        self.target.writer = writer;
        self
    }

    ///
    /// Сборка калькулятора
    ///
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
