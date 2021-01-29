use crate::tokens::Token;
use crate::ast::Ast::{Node, Leaf};
use std::fmt::{Display, Formatter, Error, Debug};


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
            Node(operation, _) => { f.write_str(&operation.to_string()); }
        }
        Ok(())
    }
}

fn to_ast(tokens: &Vec<Token>) -> Vec<Ast> {
    let open_brace_index = tokens.iter().position(|it| match it {
        Token::OpenBrace => { true }
        Token::CloseBrace => { false }
        Token::Whitespace => { false }
        Token::String(_) => { false }
    });
    match open_brace_index {
        None => {
            let mut leafs = Vec::new();
            for (_i, it) in tokens.iter().enumerate() { leafs.push(Leaf(it.clone())) }
            leafs
        }
        Some(open_brace_index) => {
            let close_brace_index = tokens.iter().position(|it| match it {
                Token::OpenBrace => { false }
                Token::CloseBrace => { true }
                Token::Whitespace => { false }
                Token::String(_) => { false }
            });

            match close_brace_index {
                None => { panic!("Missing close brace!") }
                Some(close_brace_index) => {
                    let operation = tokens.get(open_brace_index + 1).unwrap().clone();
                    let mut tail = Vec::new();
                    let token_slice = &tokens[open_brace_index+1..close_brace_index-1];
                    for (_i, it) in token_slice.iter().enumerate() { tail.push(it.clone()) }
                    vec!(Node(operation, to_ast(&tail)))
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
        let asts = to_ast(&tokens);

        assert_eq!(asts.len(), 1);
        let ast = asts.first().unwrap();
        println!("{}", ast);
        match ast {
            Leaf(_) => {
                panic!("Expected ast to be a node, but got leaf");
            }
            Node(operation, _) => {
                assert_eq!(operation, &String(std::string::String::from("+")));
            }
        }
    }
}