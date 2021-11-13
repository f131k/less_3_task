use crate::operator::{Expression, Lexem, Operator};
use crate::queue::Queue;
use crate::stack::Stack;
use crate::token::{Token, TokenList, TokenType};


///
/// Типаж для определения преобразователя
/// Принимает на вход список токенов
/// В качестве результата выдает либо очередь чисел и операторов для вычисления,
///  либо строку с описанием возникшей ошибки
///
pub trait Converter {
    fn convert(&self, input: TokenList) -> Result<Expression, &str>;
}

///
/// Пустой преобразователь.
/// Используется для создания пустого калькулятора
///
pub struct EmptyConverter {}

// Пустая реализация для пустого преобразователя
impl Converter for EmptyConverter {
    fn convert(&self, _: TokenList) -> Result<Expression, &str> {
        Err("Empty")
    }
}

///
/// Объект для преобразования входной последовательности токенов в очередь
///   соответствующей обратной польской нотации
///
pub struct InfixToRPN;

// Реализация типажа преобразования для объекта InfixToRPN
impl Converter for InfixToRPN {
    ///
    /// Фукнция преобразования из инфиксной записи в префиксную
    /// Используется алгоритм сортировочной станции Э. Дейкстра
    ///
    fn convert(&self, input: TokenList) -> Result<Expression, &str> {
        let mut stack: Stack<Token> = Stack::new();
        let mut output: Expression = Queue::new();

        for tok in input {
            match tok.0 {
                TokenType::NumberInt | TokenType::NumberFloat => {
                    // Если токен — число, то добавить его в очередь вывода
                    output.enqueue(Lexem::new(&tok));
                }
                TokenType::Function => {
                    // Если токен — функция, то поместить его в стек
                    stack.push(tok);
                }
                TokenType::ArgumentSeparator => {
                    // Если токен — разделитель аргументов функции (например запятая):
                    //     Пока токен на вершине стека не открывающая скобка:
                    //         Переложить оператор из стека в выходную очередь.
                    while let Some(last) = stack.peek() {
                        if last.0 != TokenType::OpenedParenthesis {
                            let op = stack.pop().unwrap();
                            output.enqueue(Lexem::new(&op));
                        } else {
                            break;
                        }
                    }

                    // Если стек закончился до того, как был встречен токен открывающая скобка,
                    //   то в выражении пропущен разделитель аргументов функции (запятая),
                    //   либо пропущена открывающая скобка.
                    if stack.is_empty() {
                        return Err("в выражении пропущен разделитель аргументов функции (запятая), либо пропущена открывающая скобка");
                    }
                }
                TokenType::UnaryOperator => {
                    stack.push(tok);
                }
                TokenType::BinaryOperator => {
                    // Если токен — оператор op1, то:
                    //     Пока присутствует на вершине стека токен оператор op2,
                    //       чей приоритет выше или равен приоритету op1,
                    //       и при равенстве приоритетов op1 является левоассоциативным:
                    //         Переложить op2 из стека в выходную очередь;
                    while let Some(last) = stack.peek() {
                        if Operator::get_operator(&tok) >= Operator::get_operator(&last) {
                            output.enqueue(Lexem::new(&last));
                            let _ = stack.pop();
                        } else {
                            break;
                        }
                    }

                    // Положить op1 в стек.
                    stack.push(tok);
                }
                TokenType::OpenedParenthesis => {
                    // Если токен — открывающая скобка, то положить его в стек
                    stack.push(tok);
                }
                TokenType::ClosedParenthesis => {
                    // Если токен — закрывающая скобка:
                    //     Пока токен на вершине стека не открывающая скобка
                    //         Переложить оператор из стека в выходную очередь.
                    while !stack.is_empty()
                        && stack.peek().unwrap().0 != TokenType::OpenedParenthesis
                    {
                        let op = stack.pop().unwrap();
                        output.enqueue(Lexem::new(&op));
                    }

                    // Если стек закончился до того, как был встречен токен открывающая скобка, то в выражении пропущена скобка.
                    if stack.is_empty() {
                        return Err("в выражении пропущена открывающая скобка");
                    } else {
                        // Выкинуть открывающую скобку из стека, но не добавлять в очередь вывода.
                        let _ = stack.pop();
                        // Если токен на вершине стека — функция, переложить её в выходную очередь.
                        if !stack.is_empty() && stack.peek().unwrap().0 == TokenType::Function {
                            let op = stack.pop().unwrap();
                            output.enqueue(Lexem::new(&op));
                        }
                    }
                }
                TokenType::Whitespaces => println!("А вот такого быть не должно"),
            }
        }

        // Если больше не осталось токенов на входе:
        // Пока есть токены операторы в стеке:
        while let Some(last) = stack.peek() {
            // Если токен опратор на вершине стека — открывающая скобка, то в выражении пропущена скобка.
            if last.0 == TokenType::OpenedParenthesis {
                return Err("в выражении пропущена скобка");
            }

            // Переложить оператор из стека в выходную очередь.
            let op = stack.pop().unwrap();
            output.enqueue(Lexem::new(&op));
        }

        Ok(output)
    }
}


// Базовые тесты
#[cfg(test)]
#[test]
fn test_convert() {
    let test_tokens: TokenList = vec![
        (TokenType::OpenedParenthesis, "(".to_string()),
        (TokenType::NumberInt, "1".to_string()),
        (TokenType::UnaryOperator, "+".to_string()),
        (TokenType::UnaryOperator, "-".to_string()),
        (TokenType::NumberFloat, "1.1".to_string()),
        (TokenType::ClosedParenthesis, ")".to_string()),
        (TokenType::BinaryOperator, "*".to_string()),
        (TokenType::NumberInt, "2".to_string()),
        (TokenType::BinaryOperator, "/".to_string()),
        (TokenType::NumberInt, "3".to_string()),
        (TokenType::BinaryOperator, ">>".to_string()),
        (TokenType::NumberInt, "4".to_string()),
        (TokenType::BinaryOperator, "<<".to_string()),
        (TokenType::NumberInt, "5".to_string()),
    ];

    let test_converter = InfixToRPN {};

    assert!(test_converter.convert(test_tokens).is_ok());
}

#[test]
#[ignore]
fn test_convert_miss_separator() {
    let test_tokens: TokenList = vec![
        (TokenType::Function, "fn".to_string()),
        (TokenType::OpenedParenthesis, "(".to_string()),
        (TokenType::NumberInt, "1".to_string()),
        (TokenType::ArgumentSeparator, ",".to_string()),
        (TokenType::NumberInt, "1".to_string()),
        (TokenType::NumberInt, "1".to_string()),
        (TokenType::ClosedParenthesis, ")".to_string()),
    ];

    let test_converter = InfixToRPN {};

    assert!(test_converter.convert(test_tokens).is_err());
}

#[test]
fn test_convert_miss_brace() {
    let test_tokens: TokenList = vec![
        (TokenType::OpenedParenthesis, "(".to_string()),
        (TokenType::ClosedParenthesis, ")".to_string()),
        (TokenType::ClosedParenthesis, ")".to_string()),
    ];

    let test_converter = InfixToRPN {};

    assert!(test_converter.convert(test_tokens).is_err());
}
