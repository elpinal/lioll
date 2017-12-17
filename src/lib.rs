//! The Lioll project.
#![warn(missing_docs)]

use std::io;
use std::io::Read;
use std::iter::Peekable;

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
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
