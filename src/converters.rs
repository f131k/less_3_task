use crate::token::{Token, TokenList,TokenType};
use crate::stack::Stack;
use crate::queue::Queue;
use crate::operator::{Operator, Expression, Number};

pub trait Converter {
    fn convert<'a>(&self, input: TokenList) -> Result<Expression, &str>;
}

pub struct EmptyConverter {}

impl Converter for EmptyConverter {
    fn convert<'a>(&self, _: TokenList) -> Result<Expression, &str> {
        Err("Empty")
    }
}

pub struct InfixToRPN;

impl InfixToRPN {
    // pub fn new() -> Self {
    //     InfixToRPN {
    //         out_queue: Queue::new(),
    //         tmp_stack: Stack::new(),
    //     }
    // }

    // Определяем, нужно ли выталкивать из стека имеющийся там оператор
    // fn need_op_pop_from_stack(&self, op1: &str, op2: &str) -> bool {
    //     let (op1_prio, op1_associo) = get_op_info(op1).unwrap();
    //     let (op2_prio, _) = get_op_info(op2).unwrap();
    //     // Если приоритет op2 выше или равен приоритету op1 и при этом op1 является левоассоциативным
    //     if op2_prio < op1_prio ||
    //         (op2_prio == op1_prio && op1_associo == OperatorAssociation::LeftAssociation) {
    //             return true;
    //         }

    //     false
    // }
}

impl Converter for InfixToRPN {
    fn convert<'a>(&self, input: TokenList) -> Result<Expression, &str> {
        let mut arguments: Queue<Number> = Queue::new();
        let mut stack: Stack<Token> = Stack::new();
        let mut ops : Queue<Operator> = Queue::new();

        for tok in input {
            println!("{:?} = {}", tok.0, tok.1);

            match tok.0 {
                TokenType::NumberInt | TokenType::NumberFloat => {
                    // Если токен — число, то добавить его в очередь вывода
                    let v = tok.1.parse::<Number>().unwrap();
                    arguments.enqueue(v);
                },
                TokenType::Function => {
                    // Если токен — функция, то поместить его в стек
                    stack.push(tok);
                },
                TokenType::ArgumentSeparator => {
                    // Если токен — разделитель аргументов функции (например запятая):
                    //     Пока токен на вершине стека не открывающая скобка:
                    //         Переложить оператор из стека в выходную очередь.
                    while !stack.is_empty() && stack.peek().unwrap().0 != TokenType::OpenedParenthesis {
                        let op = stack.pop().unwrap();
                        // output.enqueue(op);
                        ops.enqueue(Operator::get_operator(&op));
                    }
                    // Если стек закончился до того, как был встречен токен открывающая скобка,
                    //   то в выражении пропущен разделитель аргументов функции (запятая),
                    //   либо пропущена открывающая скобка.
                    if stack.is_empty() {
                        return Err("в выражении пропущен разделитель аргументов функции (запятая), либо пропущена открывающая скобка");
                    }
                },
                TokenType::UnaryOperator => {
                    stack.push(tok);
                }
                TokenType::BinaryOperator => {
                    // Если токен — оператор op1, то:
                    //     Пока присутствует на вершине стека токен оператор op2,
                    //       чей приоритет выше или равен приоритету op1,
                    //       и при равенстве приоритетов op1 является левоассоциативным:
                    //         Переложить op2 из стека в выходную очередь;
                    let mut last = stack.peek();
                    while last != None &&
                        Operator::get_operator(&tok) > Operator::get_operator(&last.unwrap()) { // TODO!!!
                            ops.enqueue(Operator::get_operator(&last.unwrap()));
                            let _ = stack.pop();
                            last = stack.peek();
                        }

                    // Положить op1 в стек.
                    stack.push(tok);
                },
                TokenType::OpenedParenthesis => {
                    // Если токен — открывающая скобка, то положить его в стек
                    stack.push(tok);
                },
                TokenType::ClosedParenthesis => {
                    // Если токен — закрывающая скобка:
                    //     Пока токен на вершине стека не открывающая скобка
                    //         Переложить оператор из стека в выходную очередь.
                    while !stack.is_empty() && stack.peek().unwrap().0 != TokenType::OpenedParenthesis {
                        let op = stack.pop().unwrap();
                        // output.enqueue(op);
                        ops.enqueue(Operator::get_operator(&op));
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
                            // output.enqueue(op);
                            ops.enqueue(Operator::get_operator(&op));
                        }
                    }
                },
                TokenType::Whitespaces => println!("А вот такого быть не должно"),
            }
        }

        // Если больше не осталось токенов на входе:
        // Пока есть токены операторы в стеке:
        let mut last = stack.peek();
        while last != None {
            // Если токен оператор на вершине стека — открывающая скобка, то в выражении пропущена скобка.
            if last.unwrap().0 == TokenType::OpenedParenthesis {
                return Err("в выражении пропущена скобка");
            }

            // Переложить оператор из стека в выходную очередь.
            let op = stack.pop().unwrap();
            // output.enqueue(op);
            ops.enqueue(Operator::get_operator(&op));
            last = stack.peek();
        }

        Ok((arguments, ops))
    }
}
