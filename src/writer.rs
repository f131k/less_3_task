
pub trait Writer {
    fn write(&self, output: &[u8]) -> Result<usize, &str>;
}

pub struct EmptyOutput {}

impl Writer for EmptyOutput {
    fn write(&self, _: &[u8]) -> Result<usize, &str> {
        Ok(0)
    }
}
