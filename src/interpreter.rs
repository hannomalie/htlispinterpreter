use crate::tokens::Token;
use crate::ast::Ast::*;
use crate::ast::Ast;

pub trait Interpreter {
    fn interpret(&self) -> std::string::String;
}
impl Interpreter for crate::ast::Ast {
    fn interpret(&self) -> String {
        match self {
            Leaf(token) => { token.to_string() }
            Node(tokens) => {
                let operation = tokens.first().unwrap();
                match operation {
                    Leaf(token) => {
                        match token {
                            Token::OpenBrace => { panic!("Operation may not be a open brace!") }
                            Token::CloseBrace => { panic!("Operation may not be a close brace!") }
                            Token::Whitespace => { panic!("Operation may not be a whitespace!") }
                            Token::String(value) => {
                                match value.as_str() {
                                    "+" => {
                                        let mut sum = 0;
                                        for(_, operand) in tokens[1..].iter().enumerate() {
                                            sum += operand.interpret().parse::<i32>().unwrap()
                                        }
                                        sum.to_string()
                                    }
                                    "-" => {
                                        let mut sum = tokens.get(1).unwrap().interpret().parse::<i32>().unwrap();

                                        for(_, operand) in tokens[2..].iter().enumerate() {
                                            sum -= operand.interpret().parse::<i32>().unwrap()
                                        }
                                        sum.to_string()
                                    }
                                    _ => { panic!("Unsupported operation {}", value) }
                                }
                            }
                        }
                    }
                    Node(_) => { panic!("Operation may not be a node!") }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::Tokenizer;
    use crate::ast::to_ast;

    #[test]
    fn simple_ast_is_interpreted_correctly() {
        let tokens = "(+ 1 1)".tokenize();
        let asts = to_ast(&tokens);
        let result = interpret(asts.first().unwrap());
        assert_eq!(result, "2")

    }
}