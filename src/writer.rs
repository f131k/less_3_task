extern crate termion;
use termion::color::Color;
use termion::{color, style};

pub trait Writer {
    fn write(&self, output: &[u8]) -> Result<usize, &str>;
    fn print_error(&self, output: String);
    fn print_warninig(&self, output: String);
    fn print_success(&self, output: String);
}

pub struct ConsoleOutput<'a> {
    pub error_color: &'a dyn Color,
    pub warning_color: &'a dyn Color,
    pub success_color: &'a dyn Color,
}

impl Default for ConsoleOutput<'_> {
    fn default() -> Self {
        ConsoleOutput {
            error_color: &color::LightRed,
            warning_color: &color::LightYellow,
            success_color: &color::LightGreen,
        }
    }
}

impl Writer for ConsoleOutput<'_> {
    fn write(&self, _: &[u8]) -> Result<usize, &str> {
        Ok(0)
    }

    fn print_error(&self, s: String) {
        println!("{}{}{}", color::Fg(self.error_color), s, style::Reset);
    }

    fn print_warninig(&self, s: String) {
        println!("{}{}{}", color::Fg(self.warning_color), s, style::Reset);
    }

    fn print_success(&self, s: String) {
        println!("{}{}{}", color::Fg(self.success_color), s, style::Reset);
    }
}
