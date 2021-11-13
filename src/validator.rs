use crate::token::TokenList;

///
/// Псевдоним для сигнатуры функций правил
/// Все добавляемые в валидатор функции-правила должны соответствовать этой сигнатуре
///
pub type Rule = fn(&mut TokenList) -> bool;

///
/// Объект валидатор
///
pub struct Validator {
    rules: Vec<Rule>,
}

// Реализация методов валидатора
impl Validator {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    ///
    /// Провести валидацию списка токенов
    /// Входной список токенов клонируется и после проверки возвращается новый список токенов
    ///
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

    ///
    /// Добавить правило к списку проверок
    ///
    pub fn add_rule(&mut self, r: Rule) -> &mut Self {
        self.rules.push(r);
        self
    }
}
