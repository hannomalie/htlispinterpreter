use crate::tokens::Token::{OpenBrace, CloseBrace, Whitespace, String};
use std::fmt::{Formatter, Error};

#[derive(Debug, Eq, PartialEq, Clone)]
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
    fn simple_tokenizing_works() {
        let tokens = tokenize("(+ 1 1)");
        assert_eq!(tokens.len(), 7);
        assert_eq!(tokens.get(0).unwrap(), &OpenBrace);
        assert_eq!(tokens.get(1).unwrap(), &String(std::string::String::from("+")));
        assert_eq!(tokens.get(2).unwrap(), &Whitespace);
        assert_eq!(tokens.get(3).unwrap(), &String(std::string::String::from("1")));
        assert_eq!(tokens.get(4).unwrap(), &Whitespace);
        assert_eq!(tokens.get(5).unwrap(), &String(std::string::String::from("1")));
        assert_eq!(tokens.get(6).unwrap(), &CloseBrace);
    }
    #[test]
    fn complex_tokenizing_works() {
        let tokens = tokenize("(+ 1 (- 5 2))");
        assert_eq!(tokens.len(), 13);
        assert_eq!(tokens.get(0).unwrap(), &OpenBrace);
        assert_eq!(tokens.get(1).unwrap(), &String(std::string::String::from("+")));
        assert_eq!(tokens.get(2).unwrap(), &Whitespace);
        assert_eq!(tokens.get(3).unwrap(), &String(std::string::String::from("1")));
        assert_eq!(tokens.get(4).unwrap(), &Whitespace);

        assert_eq!(tokens.get(5).unwrap(), &OpenBrace);
        assert_eq!(tokens.get(6).unwrap(), &String(std::string::String::from("-")));
        assert_eq!(tokens.get(7).unwrap(), &Whitespace);
        assert_eq!(tokens.get(8).unwrap(), &String(std::string::String::from("5")));
        assert_eq!(tokens.get(9).unwrap(), &Whitespace);
        assert_eq!(tokens.get(10).unwrap(), &String(std::string::String::from("2")));
        assert_eq!(tokens.get(11).unwrap(), &CloseBrace);

        assert_eq!(tokens.get(12).unwrap(), &CloseBrace);
    }
}