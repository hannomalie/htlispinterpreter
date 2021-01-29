mod tokens;
mod ast;

use tokens::tokenize;

fn main() {
    let tokens = tokenize("(+ 1 1)");
    for token in tokens {
        println!("{}", token);
    }

}
