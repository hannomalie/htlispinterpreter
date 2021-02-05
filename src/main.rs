mod tokens;
mod ast;
mod interpreter;

use tokens::Tokenizer;
use interpreter::Interpreter;
use crate::ast::AstSpanner;

fn main() {
    let mut input = String::new();
    loop {
        std::io::stdin().read_line(&mut input);
        let input_trimmed = input.trim();

        println!("Input: '{}'", input_trimmed);
        if input_trimmed.is_empty() { break };


        std::panic::catch_unwind(|| {
            let tokens = input_trimmed.tokenize();
            let asts = &tokens.to_ast();

            for(ast) in asts {
                println!("{}", &ast.interpret())
            }
        });

        input = String::new();

    }

}
