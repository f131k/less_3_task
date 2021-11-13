use std::rc::Rc;

use crate::converters::{Converter, EmptyConverter};
use crate::lexer::{EmptyLexer, Lexer};
use crate::operator::{Expression, Lexem, Number, Operator};
use crate::reader::{EmptyInput, Reader};
use crate::stack::Stack;
use crate::validator::Validator;
use crate::writer::{ConsoleOutput, Writer};

///
/// Объект калькулятора содержащий необходимые для работы объекты
///
pub struct Calculator {
    pub hello_str: String,
    pub input: Rc<dyn Reader>,
    pub lexer: Rc<dyn Lexer>,
    pub validator: Rc<Validator>,
    pub converter: Rc<dyn Converter>,
    pub writer: Rc<dyn Writer>,
}

/// Реализация методов объекта калькулятора
impl Calculator {
    /// Создание нового объекта
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

    /// Выполнение процедуры вычислений
    pub fn run(&mut self) {
        println!("{}", self.hello_str);

        // Получение входной строки
        let mut input_string = match self.input.read() {
            Ok(result) => result,
            Err(why) => {
                self.writer
                    .print_error(format!("Ошибка получения входной строки: {}", why));
                return;
            }
        };

        // Разбор на токены (лексемы)
        let tokens = match self.lexer.tokenize(&mut input_string) {
            Ok(result) => result,
            Err(why) => {
                self.writer.print_error(format!(
                    "Ошибка разбиения на лексемы:\n{0}\n {2:>1$} неизвестная лексема!",
                    input_string,
                    input_string.find(why).unwrap(),
                    "^"
                ));
                return;
            }
        };

        // Валидация по установленным правилам
        let valid_tokens = match self.validator.validate(tokens) {
            Ok(result) => result,
            Err(why) => {
                self.writer
                    .print_error(format!("Ошибка валидации: {}", why));
                return;
            }
        };

        // Преобразование входной последовательности токенов
        let mut expr = match self.converter.convert(valid_tokens) {
            Ok(result) => result,
            Err(why) => {
                self.writer
                    .print_error(format!("Ошибка преобразования: {}", why));
                return;
            }
        };

        // Вычисление выражения по преобразованной последовательности
        let res = match self.calculate(&mut expr) {
            Ok(result) => result,
            Err(why) => {
                self.writer
                    .print_error(format!("Ошибка вычисления : {}", why));
                return;
            }
        };

        // Вывод результата
        self.writer
            .print_success(format!("\nРезультат выражения: {}", res));
    }

    ///
    /// Вычисление выражения для обратной польской нотации
    /// Использует стандартный алгоритм с использованием стека
    ///
    pub fn calculate(&self, input: &mut Expression) -> Result<Number, &str> {
        let mut arguments_stack: Stack<Number> = Stack::new();

        while let Some(lexem) = input.dequeue() {
            match lexem {
                Lexem::NumberLex(v) => {
                    print!("{} ", v);
                    arguments_stack.push(v);
                }

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
                        }
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
                        }
                        _ => continue,
                    };
                }
            };
        }

        if let Some(result) = arguments_stack.pop() {
            if !arguments_stack.is_empty() {
                return Err("Очередь аргументов не пуста, но очередь операторов опустела");
            }

            return Ok(result);
        }

        Err("Не удалось вычислить выражение")
    }
}

// базовые тесты
#[cfg(test)]
use crate::queue::Queue;
use crate::token::{TokenType, Token};

#[test]
fn test_calculate_simple() {
    let mut expr: Queue<Lexem> = Queue::new();
    expr.enqueue(Lexem::NumberLex(1.0));
    expr.enqueue(Lexem::NumberLex(1.0));

    let op : Token = (TokenType::BinaryOperator, "+".to_string());
    expr.enqueue(Lexem::OperatorLex(Operator::get_operator(&op)));

    let clc: Calculator = Calculator::new();
    assert_eq!(clc.calculate(&mut expr), Ok(2.0));
}

#[test]
fn test_calculate_two_plus_two_mult_two() {
    // проверка приоритетов операторов через решение 2+2*2 => 2 2 2 * +
    let mut expr: Queue<Lexem> = Queue::new();
    expr.enqueue(Lexem::NumberLex(2.0));
    expr.enqueue(Lexem::NumberLex(2.0));
    expr.enqueue(Lexem::NumberLex(2.0));

    let op1 : Token = (TokenType::BinaryOperator, "*".to_string());
    expr.enqueue(Lexem::OperatorLex(Operator::get_operator(&op1)));

    let op2 : Token = (TokenType::BinaryOperator, "+".to_string());
    expr.enqueue(Lexem::OperatorLex(Operator::get_operator(&op2)));

    let clc: Calculator = Calculator::new();
    assert_eq!(clc.calculate(&mut expr), Ok(6.0));

}
