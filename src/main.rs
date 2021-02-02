mod tokens;
mod ast;
mod interpreter;

use tokens::tokenize;
use ast::to_ast;
use interpreter::interpret;

fn main() {
    let mut input = String::new();
    loop {
        std::io::stdin().read_line(&mut input);
        let input_trimmed = input.trim();

        println!("Input: '{}'", input_trimmed);
        if input_trimmed.is_empty() { break };


        std::panic::catch_unwind(|| {
            let tokens = tokenize(input_trimmed);
            let asts = to_ast(&tokens);

            for(ast) in asts {
                println!("{}", interpret(&ast))
            }
        });

        input = String::new();

    }

}
