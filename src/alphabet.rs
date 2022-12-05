use crate::error::Error;

pub trait Alphabet {
    const PADDING: char;

    fn char(&self, index: u8) -> char;
    fn index(&self, char: char) -> Result<u8, Error>;
}
