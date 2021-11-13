use std::io;
use std::io::Write;

///
/// Типаж для определения объектов получающих входную строку
///
pub trait Reader {
    fn read(&self) -> Result<String, String>;
}

// Объект заглушка
pub struct EmptyInput {}
impl Reader for EmptyInput {
    fn read(&self) -> Result<String, String> {
        Ok(String::new())
    }
}

// Объект получающий строку из стандартного потока ввода
pub struct ConsoleReader;

// Реализация типажа Readed
impl Reader for ConsoleReader {
    ///
    /// Чтение строки из стандартного потока ввода
    ///
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
