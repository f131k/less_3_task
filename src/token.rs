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

// Псевдоним кортежа для удобства работы - (Тип токена, "символьное представление")
pub type Token = (TokenType, String);

// Псевдоним для краткости записи
pub type TokenList = Vec<Token>;
