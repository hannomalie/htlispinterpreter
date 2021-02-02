use crate::tokens::Token;
use crate::ast::Ast::{Node, Leaf};
use std::fmt::{Display, Formatter, Error, Debug};


#[derive(Debug)]
pub enum Ast {
    Leaf(Token),
    Node(Vec<Ast>) // TODO: Make operation a single property, not part of vec
}
impl Display for Ast {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        // TODO: Don't ignore results
        match self {
            Leaf(token) => { f.write_str(&token.to_string()); }
            Node(tokens) => { f.write_str(&tokens.first().unwrap().to_string()); }
        }
        Ok(())
    }
}

pub fn to_ast(tokens: &Vec<Token>) -> Vec<crate::ast::Ast> {

    let mut scopes: Vec<crate::ast::Ast> = Vec::new();
    let mut brace_balance = 0;

    for (_i, token) in tokens.iter().enumerate() {
        match token {
            Token::OpenBrace => {
                scopes.push(Node(Vec::new()));
                brace_balance+=1;
            }
            Token::CloseBrace => {
                if !scopes.is_empty() {
                    let last_scope = scopes.remove(scopes.len()-1);
                    match scopes.last_mut() {
                        None => { return vec!(last_scope) }
                        Some(scope_before) => {
                            match scope_before {
                                Leaf(_) => { panic!("Scope before is a leaf!") }
                                Node(tokens) => { tokens.push(last_scope) }
                            }
                        }
                    }
                }
                brace_balance-=1;
            }
            Token::Whitespace => {}
            Token::String(_) => {
                match scopes.last_mut() {
                    None => { panic!("No opening brace found!") }
                    Some(scope) => {
                        match scope {
                            Leaf(_) => { panic!("Cannot add token to leaf!") }
                            Node(tokens) => {
                                tokens.push(Leaf(token.clone()));
                            }
                        }
                    }
                }
            }
        }
    }


    if brace_balance != 0 { panic!("Unequal amount of braces!"); }

    scopes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::tokenize;
    use crate::tokens::Token::String;

    #[test]
    fn simple_ast_is_parsed_correctly() {
        let tokens = tokenize("(+ 1 1)");
        assert_eq!(tokens.len(), 7);
        let asts = to_ast(&tokens);

        assert_eq!(asts.len(), 1);
        let ast = asts.first().unwrap();
        match ast {
            Leaf(_) => {
                panic!("Expected ast to be a node, but got leaf");
            }
            Node(elements) => {
                let operation = elements.first().unwrap();
                match operation {
                    Leaf(token) => { assert_eq!(token, &String(std::string::String::from("+"))); }
                    Node(_) => { panic!("Expected operation to be a leaf, but got node") }
                }
                assert_eq!(elements.len(), 3);
                assert_eq!(match elements.get(1).unwrap() {
                    Leaf(token) => { token }
                    Node(_) => { panic!("Expected operand to be a leaf, but got node") }
                }, &String(std::string::String::from("1")));
                assert_eq!(match elements.get(2).unwrap() {
                    Leaf(token) => { token }
                    Node(_) => { panic!("Expected operand to be a leaf, but got node") }
                }, &String(std::string::String::from("1")));
            }
        }
    }

    #[test]
    fn complex_ast_is_parsed_correctly() {
        let tokens = tokenize("(+ 1 (- 5 2))");
        assert_eq!(tokens.len(), 13);
        let asts = to_ast(&tokens);

        assert_eq!(asts.len(), 1);
        let ast = asts.first().unwrap();
        match ast {
            Leaf(_) => {
                panic!("Expected ast to be a node, but got leaf");
            }
            Node(elements) => {
                let operation = elements.first().unwrap();
                match operation {
                    Leaf(token) => { assert_eq!(token, &String(std::string::String::from("+"))); }
                    Node(_) => { panic!("Expected operation to be a leaf, but got node") }
                }
                assert_eq!(elements.len(), 3);

                assert_eq!(match elements.get(1).unwrap() {
                    Leaf(token) => { token }
                    Node(_) => { panic!("Expected operand to be a leaf, but got node") }
                }, &String(std::string::String::from("1")));

                match elements.get(2).unwrap() {
                    Leaf(_) => { panic!("Expected operand to be a node, but got leaf") }
                    Node(elements) => {
                        let operation = elements.first().unwrap();
                        match operation {
                            Leaf(token) => { assert_eq!(token, &String(std::string::String::from("-"))); }
                            Node(_) => { panic!("Expected operation to be a leaf, but got node") }
                        }

                        assert_eq!(match elements.get(1).unwrap() {
                            Leaf(token) => { token }
                            Node(_) => { panic!("Expected operand to be a leaf, but got node") }
                        }, &String(std::string::String::from("5")));

                        assert_eq!(match elements.get(2).unwrap() {
                            Leaf(token) => { token }
                            Node(_) => { panic!("Expected operand to be a leaf, but got node") }
                        }, &String(std::string::String::from("2")));
                    }
                };
            }
        }
    }
}