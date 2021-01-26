use crate::tokens::Token::{OpenBrace, CloseBrace, Whitespace, String};
use std::fmt::{Formatter, Error};

#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    Whitespace,
    String(std::string::String)
}
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            OpenBrace => { f.write_str("OpenBrace") }
            CloseBrace => { f.write_str("CloseBrace") }
            Whitespace => { f.write_str("WhiteSpace") }
            String(value) => { f.write_str(value) }
        }
    }
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    for (_i, &item) in input.as_bytes().iter().enumerate() {
        if item == b'(' {
            tokens.push(OpenBrace)
        } else if item == b')' {
            tokens.push(CloseBrace)
        } else if item == b' ' {
            tokens.push(Whitespace)
        } else { tokens.push(String(std::string::String::from_utf8(vec![item]).unwrap())) }
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let tokens = tokenize("(+ 1 1)");
        assert_eq!(tokens.len(), 7);
        assert_eq!(tokens.get(0).unwrap(), &OpenBrace);
        assert_eq!(tokens.get(1).unwrap(), "+");
        assert_eq!(tokens.get(2).unwrap(), &Whitespace);
        assert_eq!(tokens.get(3).unwrap(), "1");
        assert_eq!(tokens.get(4).unwrap(), &Whitespace);
        assert_eq!(tokens.get(5).unwrap(), "1");
        assert_eq!(tokens.get(6).unwrap(), &CloseBrace);
    }
}