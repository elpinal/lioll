//! The Lioll project.
#![warn(missing_docs)]

use std::io;
use std::io::Read;
use std::iter::Peekable;
use std::string::FromUtf8Error;

/// Represents tokens.
#[derive(Debug, PartialEq)]
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
        loop {
            match self.bytes.peek() {
                None => return Err(LexError::EOF),
                Some(r) => {
                    if let &Ok(b) = r {
                        match b {
                            b'[' => return Ok(Token::LBrack),
                            b']' => return Ok(Token::RBrack),
                            b'\'' => return self.string(),
                            _ => (),
                        }
                    }
                }
            }
            self.bytes.next().unwrap()?;
        }
    }

    fn string(&mut self) -> Result<Token, LexError> {
        match self.bytes.next() {
            None => return Err(LexError::Terminate),
            Some(r) => unimplemented!(),
        }
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

impl LexError {
    /// Test an error is caused by EOF.
    pub fn is_eof(&self) -> bool {
        match self {
            &LexError::EOF => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex() {
        let mut l = Lexer::new("[".as_bytes());
        assert_eq!(l.lex().ok(), Some(Token::LBrack));
        assert!(l.lex().unwrap_err().is_eof());
    }
}
