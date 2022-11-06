mod lexer;
mod parser;

use lexer::Lexer;
use parser::parser;

use ::std::fs::read_to_string;

fn main() {
    let test_file = read_to_string("./test_files/HelloWorld.appian");

    if let Ok(str) = test_file {
        println!("{:?}", parser(Lexer::from(str.as_str()).collect()))
    }
}
