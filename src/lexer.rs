use std::str::Chars;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Token {
    Word(String),
    NamespaceSeparator,
    LineSeparator,
    NameSeparator,
    OpenObject,
    CloseObject,
    OpenArguments,
    CloseArguments,
}

pub fn lexer(str: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    lexerino(&mut str.chars(), &mut tokens);
    tokens
}

fn lexerino(char_iterator: &mut Chars, tokens: &mut Vec<Token>) {
    if let Some(char) = char_iterator.next() {
        match char {
            ',' => tokens.push(Token::LineSeparator),
            '(' => tokens.push(Token::OpenArguments),
            ')' => tokens.push(Token::CloseArguments),
            '{' => tokens.push(Token::OpenObject),
            '}' => tokens.push(Token::CloseObject),
            ':' => tokens.push(Token::NameSeparator),
            '!' => tokens.push(Token::NamespaceSeparator),
            c if c.is_whitespace() => (),
            _ => unimplemented!(),
        };
        lexerino(char_iterator, tokens);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lexer_with_empty_input_should_return_empty_vector() {
        assert_eq!(0, lexer("").len())
    }

    #[test]
    fn lexer_with_all_the_syntax_element_as_input_should_return_a_vector_with_all_those_elements() {
        let expected_vector = Vec::from([
            Token::OpenObject,
            Token::CloseObject,
            Token::OpenArguments,
            Token::CloseArguments,
            Token::NamespaceSeparator,
            Token::LineSeparator,
            Token::NameSeparator,
        ]);

        assert_eq!(expected_vector, lexer("{}()!,:"))
    }

    #[test]
    fn lexer_should_ignore_whitespaces() {
        let expected_vector = Vec::from([
            Token::OpenObject,
            Token::CloseObject,
            Token::OpenArguments,
            Token::CloseArguments,
            Token::NamespaceSeparator,
            Token::LineSeparator,
            Token::NameSeparator,
        ]);

        assert_eq!(expected_vector, lexer("{}   ()    !  \n\t ,:\r"))
    }

    #[test]
    fn lexer_should_retrieve_unsyntaxic_characters_as_a_word() {
        let expected_vector = Vec::from([Token::Word(String::from("Test"))]);

        assert_eq!(expected_vector, lexer("Test"))
    }
}
