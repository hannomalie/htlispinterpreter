use crate::Token::{OpenBrace, Whitespace, CloseBrace, Char};

fn main() {
    println!("Hello, world!");
}
#[derive(Debug, Eq, PartialEq)]
enum Token {
    OpenBrace,
    CloseBrace,
    Whitespace,
    Char(u8)
}

fn read(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    for (i, &item) in input.as_bytes().iter().enumerate() {
        if item == b'(' {
            tokens.push(OpenBrace)
        } else if item == b')' {
            tokens.push(CloseBrace)
        } else if item == b' ' {
            tokens.push(Whitespace)
        } else { tokens.push(Char(item)) }
    }
    tokens
}

#[cfg(test)]
mod tests {
    use crate::read;
    use crate::Token::{OpenBrace, Char, CloseBrace, Whitespace};

    #[test]
    fn it_works() {
        let tokens = read("(+ 1 1)");
        assert_eq!(tokens.len(), 7);
        assert_eq!(tokens.get(0).unwrap(), &OpenBrace);
        assert_eq!(tokens.get(1).unwrap(), &Char(b'+'));
        assert_eq!(tokens.get(2).unwrap(), &Whitespace);
        assert_eq!(tokens.get(3).unwrap(), &Char(b'1'));
        assert_eq!(tokens.get(4).unwrap(), &Whitespace);
        assert_eq!(tokens.get(5).unwrap(), &Char(b'1'));
        assert_eq!(tokens.get(6).unwrap(), &CloseBrace);
    }
}
