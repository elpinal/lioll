//! The Lioll project.
#![warn(missing_docs)]

use std::io;
use std::io::Read;
use std::iter::Peekable;
use std::string::FromUtf8Error;

/// Represents tokens.
enum Token {
    String(String),
    Number(isize),
    LBrack,
    RBrack,
}

/// A lexer.
struct Lexer<R: Read + Sized> {
    bytes: Peekable<io::Bytes<R>>,
}

impl<R> Lexer<R>
where
    R: Read,
{
    /// Creates a new lexer.
    pub fn new(r: R) -> Lexer<R> {
        Lexer { bytes: r.bytes().peekable() }
    }

    /// Lexes a token. It returns an error if the input is invalid.
    pub fn lex(&mut self) -> Result<Token, LexError> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub enum LexError {
    /// Error on IO.
    IO(io::Error),
    /// Invalid UTF-8 error.
    Utf8(FromUtf8Error),
    /// An error that indicates "string literal is not terminated."
    Terminate,
    /// End of file.
    EOF,
    /// Illegal byte.
    Illegal,
}

impl From<io::Error> for LexError {
    fn from(e: io::Error) -> LexError {
        LexError::IO(e)
    }
}

impl From<FromUtf8Error> for LexError {
    fn from(e: FromUtf8Error) -> LexError {
        LexError::Utf8(e)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
