use crate::token::TokenList;

pub type Rule = fn(&mut TokenList) -> bool;

pub struct Validator {
    rules: Vec<Rule>,
}

impl Validator {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn validate(&self, input: TokenList) -> Result<TokenList, &str> {
        if self.rules.is_empty() {
            return Ok(input);
        }

        let mut output: TokenList = input.clone();
        for rule in &self.rules {
            if !rule(&mut output) {
                return Err("Ошибка валидации");
            }
        }

        Ok(output)
    }

    pub fn add_rule(&mut self, r: Rule) -> &mut Self {
        self.rules.push(r);
        self
    }
}
