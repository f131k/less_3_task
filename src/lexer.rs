//!
use regex::Regex;

use crate::token::{TokenList, TokenType};

pub trait Lexer {
    fn tokenize(&self, input: &str) -> Result<TokenList, char>;
}

pub struct EmptyLexer {}

impl Lexer for EmptyLexer {
    fn tokenize(&self, _: &str) -> Result<TokenList, char> {
        Err(' ')
    }
}

pub struct RegexpLexer {
    knows_tokens: Vec<(TokenType, Regex)>,
}

impl RegexpLexer {
    pub fn new() -> Self {
        let mut list: Vec<(TokenType, Regex)> = Vec::new();
        list.push((TokenType::OpenedParenthesis, Regex::new(r"^(\()").unwrap()));
        list.push((TokenType::ClosedParenthesis, Regex::new(r"^(\))").unwrap()));
        list.push((TokenType::Function, Regex::new(r"^[a-zA-Z]+").unwrap()));
        list.push((
            TokenType::BinaryOperator,
            Regex::new(r"^(([/\*\^]{1,1})|(<{2,2})|(>{2,2}))").unwrap(),
        ));
        list.push((
            TokenType::UnaryOperator,
            Regex::new(r"^([\+\-]{1,1})").unwrap(),
        ));
        list.push((TokenType::NumberFloat, Regex::new(r"^(\d+\.\d+)").unwrap()));
        list.push((TokenType::NumberInt, Regex::new(r"^(\d+)").unwrap()));
        list.push((
            TokenType::ArgumentSeparator,
            Regex::new(r"^(,{1,1})").unwrap(),
        ));
        list.push((TokenType::Whitespaces, Regex::new(r"^(\s+)").unwrap()));

        RegexpLexer { knows_tokens: list }
    }
}

impl Lexer for RegexpLexer {
    fn tokenize(&self, input: &str) -> Result<TokenList, char> {
        let mut tokens: TokenList = Vec::new();
        let mut target_string: String = input.to_string();
        let mut error: bool = false;

        while !target_string.is_empty() && !error {
            let strlen_before = target_string.len();
            for tok in &self.knows_tokens {
                let rgx: &Regex = &tok.1;
                if let Some(captions) = rgx.captures(&target_string) {
                    let value = &captions[0];
                    tokens.push((tok.0, value.to_string()));
                    target_string = target_string.strip_prefix(value).unwrap().to_string();
                    break;
                }
            }

            error = strlen_before == target_string.len();
        }

        if error {
            return Err(target_string.chars().next().unwrap());
        }

        Ok(tokens)
    }
}

#[cfg(test)]
use std::collections::HashMap;

#[test]
fn test_lexer_unkown_tokens() {
    let mut test_str = HashMap::new();
    test_str.insert('&', "1      123 123123 & 123 213");
    test_str.insert('=', "1      1 = 123123 & 123 213");
    test_str.insert('!', "1      !13 123123 & 123 213");
    test_str.insert('#', "1      1#3 123123 & 123 213");
    test_str.insert(';', "1      ;23 123123 & 123 213");
    test_str.insert('%', "1      123 123123 % 123 213");

    let lex: RegexpLexer = RegexpLexer::new();
    for (r, s) in test_str {
        assert_eq!(lex.tokenize(s), Err(r));
    }
}

#[test]
fn lexer_all_known_tokens() {
    let test_str = "(1+-1.1)*2/3>>4<<5";
    let expected: TokenList = vec![
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
    let lex: RegexpLexer = RegexpLexer::new();

    assert_eq!(lex.tokenize(test_str), Ok(expected));
}
