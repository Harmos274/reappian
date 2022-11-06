use std::{iter::Peekable, str::Chars};

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Token {
    Word(String),
    StringLiteralSeparator,
    NamespaceSeparator,
    LineSeparator,
    NameSeparator,
    OpenObject,
    CloseObject,
    OpenArguments,
    CloseArguments,
}

pub struct Lexer<T: Iterator<Item = char>>(Peekable<T>, LexingState);

enum LexingState {
    CollectingStringLiteral,
    CollectedStringLiteral,
    Basic,
}

impl<T> Iterator for Lexer<T>
where
    T: Iterator<Item = char>,
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.1 {
            LexingState::CollectedStringLiteral => self.collected_string_literal(),
            LexingState::CollectingStringLiteral => self.collecting_string_literal(),
            LexingState::Basic => self.basic(),
        }
    }
}

impl<T> Lexer<T>
where
    T: Iterator<Item = char>,
{
    const STRING_LITERAL_SEPARATOR: char = '"';
    const LINE_SEPARATOR: char = ',';
    const OPEN_ARGUMENTS: char = '(';
    const CLOSE_ARGUMENTS: char = ')';
    const OPEN_OBJECT: char = '{';
    const CLOSE_OBJECT: char = '}';
    const NAME_SEPARATOR: char = ':';
    const NAMESPACE_SEPARATOR: char = '!';

    pub fn new(stream: T) -> Self {
        Self(stream.peekable(), LexingState::Basic)
    }

    fn basic(&mut self) -> Option<Token> {
        Some(match self.get_next_non_whitespace_character()? {
            Self::LINE_SEPARATOR => Token::LineSeparator,
            Self::OPEN_ARGUMENTS => Token::OpenArguments,
            Self::CLOSE_ARGUMENTS => Token::CloseArguments,
            Self::OPEN_OBJECT => Token::OpenObject,
            Self::CLOSE_OBJECT => Token::CloseObject,
            Self::NAME_SEPARATOR => Token::NameSeparator,
            Self::NAMESPACE_SEPARATOR => Token::NamespaceSeparator,
            Self::STRING_LITERAL_SEPARATOR => {
                self.1 = LexingState::CollectingStringLiteral;
                Token::StringLiteralSeparator
            }
            c => Token::Word(self.collect_word(c)),
        })
    }

    fn collecting_string_literal(&mut self) -> Option<Token> {
        self.1 = LexingState::CollectedStringLiteral;
        Some(Token::Word(self.collect_string_litteral()))
    }

    fn collected_string_literal(&mut self) -> Option<Token> {
        self.1 = LexingState::Basic;

        if *self.0.peek()? == Self::STRING_LITERAL_SEPARATOR {
            self.0.next()?;
            Some(Token::StringLiteralSeparator)
        } else {
            self.next()
        }
    }

    fn get_next_non_whitespace_character(&mut self) -> Option<char> {
        loop {
            match self.0.next()? {
                c if c.is_whitespace() => (),
                c => break Some(c),
            }
        }
    }

    fn collect_word(&mut self, first_character: char) -> String {
        let mut word = String::from(first_character);

        while let Some(&peek) = self.0.peek() {
            if Self::is_syntaxic_token(peek) {
                break;
            } else {
                word.push(peek);
                self.0.next();
            }
        }
        word
    }

    fn collect_string_litteral(&mut self) -> String {
        let mut word = String::new();

        while let Some(&peek) = self.0.peek() {
            if peek == Self::STRING_LITERAL_SEPARATOR {
                break;
            } else {
                word.push(peek);
                self.0.next();
            }
        }
        word
    }

    fn is_syntaxic_token(c: char) -> bool {
        matches!(
            c,
            Self::NAME_SEPARATOR
                | Self::OPEN_ARGUMENTS
                | Self::CLOSE_ARGUMENTS
                | Self::OPEN_OBJECT
                | Self::CLOSE_OBJECT
                | Self::LINE_SEPARATOR
                | Self::NAMESPACE_SEPARATOR
        )
    }
}

impl<'a> From<&'a str> for Lexer<Chars<'a>> {
    fn from(str: &'a str) -> Self {
        Self::new(str.chars())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn lexer(str: &str) -> Vec<Token> {
        Lexer::from(str).collect()
    }

    #[test]
    fn empty_input_should_return_empty_vector() {
        assert_eq!(0, lexer("").len())
    }

    #[test]
    fn all_the_syntax_element_as_input_should_return_a_vector_with_all_those_elements() {
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
    fn should_ignore_whitespaces() {
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
    fn should_retrieve_unsyntaxic_characters_as_a_word() {
        let expected_vector = Vec::from([Token::Word(String::from("Test"))]);

        assert_eq!(expected_vector, lexer("Test"))
    }

    #[test]
    fn should_take_empty_string_litterals() {
        let expected_vector = Vec::from([
            Token::StringLiteralSeparator,
            Token::Word("".to_string()),
            Token::StringLiteralSeparator,
        ]);

        assert_eq!(expected_vector, lexer("\"\""))
    }

    #[test]
    fn should_take_string_litterals() {
        let expected_vector = Vec::from([
            Token::StringLiteralSeparator,
            Token::Word("toto".to_string()),
            Token::StringLiteralSeparator,
        ]);

        assert_eq!(expected_vector, lexer("\"toto\""))
    }
}
