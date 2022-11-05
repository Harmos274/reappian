mod lexer;
mod parser;

use lexer::lexer;
use parser::parser;

use ::std::fs::read_to_string;

fn main() {
    let test_file = read_to_string("./test_files/HelloWorld.appian");

    if let Ok(str) = test_file {
        parser(lexer(&str))
    }
}
