//!

use regex::Regex;

use crate::token::{TokenList, TokenType};

pub trait Lexer {
    fn tokenize<'a>(&self, input: &'a mut str) -> Result<TokenList, char>;
}

pub struct EmptyLexer {}

impl Lexer for EmptyLexer {
    fn tokenize<'a>(&self, _: &'a mut str) -> Result<TokenList, char> {
        Err(' ')
    }
}

pub struct RegexpLexer {
    knows_tokens: Vec<(TokenType, Regex)>,
}

impl RegexpLexer {
    pub fn new() -> Self {
        let mut list : Vec<(TokenType, Regex)> = Vec::new();
        list.push((TokenType::OpenedParenthesis,
                   Regex::new(r"^(\()").unwrap()));
        list.push((TokenType::ClosedParenthesis,
                   Regex::new(r"^(\))").unwrap()));
        list.push((TokenType::Function,
                   Regex::new(r"^[a-zA-Z]+").unwrap()));
        list.push((TokenType::BinaryOperator,
                   Regex::new(r"^([/\*]{1,1})|(<{2,2})|(>{2,2})").unwrap()));
        list.push((TokenType::UnaryOperator,
                   Regex::new(r"^([\+\-\^]{1,1})").unwrap()));
        list.push((TokenType::NumberFloat,
                   Regex::new(r"^(\d+\.\d+)").unwrap()));
        list.push((TokenType::NumberInt,
                   Regex::new(r"^(\d+)").unwrap()));
        list.push((TokenType::ArgumentSeparator,
                   Regex::new(r"^(,{1,1})").unwrap()));
        list.push((TokenType::Whitespaces,
                   Regex::new(r"^(\s+)").unwrap()));

        RegexpLexer {
            knows_tokens: list,
        }
    }
}

impl Lexer for RegexpLexer {
    fn tokenize<'a>(&self, input: &'a mut str) -> Result<TokenList, char> {
        let mut tokens : TokenList = Vec::new();
        let mut target_string: String = input.to_string();
        let mut error : bool = false;

        while !target_string.is_empty() && !error {
            let strlen_before = target_string.len();
            for tok in &self.knows_tokens {
                let rgx : &Regex = &tok.1;
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
