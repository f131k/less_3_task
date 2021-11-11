use std::io;
use std::io::Write;

pub trait Reader {
    fn read(&self) -> Result<String, String>;
}

pub struct EmptyInput {}
impl Reader for EmptyInput {
    fn read(&self) -> Result<String, String> {
        Ok(String::new())
    }
}

pub struct ConsoleReader;

impl Reader for ConsoleReader {
    fn read(&self) -> Result<String, String> {
        let stdin = io::stdin();
        let mut input = String::new();
        print!("Введите выражение: ");
        io::stdout().flush().unwrap();
        // После последнего match сознательно пропускаем ';'
        // т.к. это должно быть выражением для возврата значения из функции
        match stdin.read_line(&mut input) {
            Ok(_) => Ok(input.trim().replace(" ", "").to_string()),
            Err(_) => Err(String::from("Не удалось прочитать строку")),
        }
    }
}
