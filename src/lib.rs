#![warn(missing_docs)]

/// Represents tokens.
enum Token {
    String(String),
    Number(isize),
    LBrack,
    RBrack,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
