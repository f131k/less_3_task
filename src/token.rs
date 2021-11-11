// pub struct Token {}

// Типы доступных токенов (лексем)
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    NumberInt,
    NumberFloat,
    UnaryOperator,
    BinaryOperator,
    Function,
    OpenedParenthesis,
    ClosedParenthesis,
    ArgumentSeparator,
    Whitespaces,
}

// Определим кортеж для удобства работы - (Тип токена, "символьное представление")
pub type Token = (TokenType, String);

pub type TokenList = Vec<Token>;
