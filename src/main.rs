
use crate::lexer::lexer::tokenize;

mod lexer;

fn main() {
    let source_code = "let x = 45 * (4 / 3)";
    println!("result: \n {:?}", tokenize(source_code));
}
