use crate::tokens::Token;
use crate::ast::Ast::{Node, Leaf};
use std::fmt::{Display, Formatter, Error, Write, Debug};


#[derive(Debug)]
enum Ast {
    Leaf(Token),
    Node(Token, Vec<Ast>)
}
impl Display for Ast {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        // TODO: Don't ignore results
        match self {
            Leaf(token) => { f.write_str(&token.to_string()); }
            Node(operation, operands) => { f.write_str(&operation.to_string()); }
        }
        Ok(())
    }
}

fn to_ast(tokens: &Vec<Token>) -> Ast { // TODO: Return Vec
    let open_brace_index = tokens.iter().position(|it| match it {
        Token::OpenBrace => { true }
        Token::CloseBrace => { false }
        Token::Whitespace => { false }
        Token::String(_) => { false }
    });
    match open_brace_index {
        None => { Leaf(tokens.first().unwrap().clone()) } // TODO: This is wrong
        Some(open_brace_index) => {
            let close_brace_index = tokens.iter().position(|it| match it {
                Token::OpenBrace => { false }
                Token::CloseBrace => { true }
                Token::Whitespace => { false }
                Token::String(_) => { false }
            });

            match close_brace_index {
                None => { Leaf(tokens.first().unwrap().clone()) } // TODO: This is wrong
                Some(close_brace_index) => {
                    let operation = tokens.get(open_brace_index + 1).unwrap().clone();
                    let mut tail = Vec::new();
                    let token_slice = &tokens[open_brace_index..close_brace_index];
                    for (_i, it) in token_slice.iter().enumerate() { tail.push(it.clone()) }
                    Node(operation, vec!(to_ast(&tail)))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::tokenize;
    use crate::tokens::Token::String;

    #[test]
    fn ast_is_parsed_correctly() {
        let tokens = tokenize("(+ 1 1)");
        let ast = to_ast(&tokens);

        println!("{}", ast);
        match ast {
            Leaf(token) => {
                panic!("Expected ast to be a node, but got leaf");
            }
            Node(operation, operands) => {
                assert_eq!(operation, String(std::string::String::from("+")));
            }
        }
    }
}