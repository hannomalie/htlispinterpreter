use crate::tokens::Token;
use crate::ast::Ast::{Node, Leaf};
use std::fmt::{Display, Formatter, Error, Debug};
use crate::ast::AstError::UnbalancedBraces;


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

pub enum AstError {
    UnbalancedBraces { tokens: Vec<Ast>, expected_brace_after_token: usize }
}

pub trait AstSpanner {
    fn to_ast(&self) -> Result<Vec<crate::ast::Ast>, AstError>;
}
impl AstSpanner for Vec<Token> {
    fn to_ast(&self) -> Result<Vec<crate::ast::Ast>, AstError> {

        let mut scopes: Vec<crate::ast::Ast> = Vec::new();
        let mut brace_balance = 0;

        for (index, token) in self.iter().enumerate() {
            match token {
                Token::OpenBrace => {
                    scopes.push(Node(Vec::new()));
                    brace_balance+=1;
                }
                Token::CloseBrace => {
                    if !scopes.is_empty() {
                        let last_scope = scopes.remove(scopes.len()-1);
                        match scopes.last_mut() {
                            None => { return Ok(vec!(last_scope)) }
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
                        None => {
                            return Err(UnbalancedBraces {
                                tokens: scopes,
                                expected_brace_after_token: index
                            })
                        }
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


        return if brace_balance != 0 {
            Err(UnbalancedBraces {
                tokens: scopes,
                expected_brace_after_token: self.len()
            })
        } else { Ok(scopes) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::Tokenizer;
    use crate::tokens::Token::String;

    #[test]
    fn simple_ast_is_parsed_correctly() {
        let tokens = "(+ 1 1)".tokenize();
        assert_eq!(tokens.len(), 7);
        let asts = &tokens.to_ast().ok().unwrap();

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
    fn unequal_braces_are_recognized_correctly() {
        let tokens = "(+ 1 1".tokenize();
        let error = &tokens.to_ast().err().unwrap();

        match error {
            AstError::UnbalancedBraces { tokens: asts, expected_brace_after_token } => {
                assert_eq!(expected_brace_after_token, &6);
                match asts.first().unwrap() {
                    Leaf(_) => { panic!("Expected ast to be a node, but got leaf"); }
                    Node(asts) => {
                        match asts.first().unwrap() {
                            Leaf(token) => {
                                assert_eq!(token, &String(std::string::String::from("+")));
                            }
                            Node(_) => { panic!("Expected ast to be a leaf, but got node"); }
                        }
                    }
                };
            }
        }
    }

    #[test]
    fn complex_ast_is_parsed_correctly() {
        let tokens = "(+ 1 (- 5 2))".tokenize();
        assert_eq!(tokens.len(), 13);
        let asts = &tokens.to_ast().ok().unwrap();

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