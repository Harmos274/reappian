#[derive(Debug)]
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

// ugly shit omg use recursivity idiot !!!!
pub fn lexer(str: &str) -> Vec<Token> {
    let (tokens, _) =
        str.chars()
            .fold((vec![], Some(String::new())), |(mut tokens, word), char| {
                match (char, word) {
                    (',', None) => tokens.push(Token::LineSeparator),
                    (',', Some(word)) => {
                        tokens.push(Token::Word(word));
                        tokens.push(Token::LineSeparator);
                    }
                    ('(', None) => tokens.push(Token::OpenArguments),
                    ('(', Some(word)) => {
                        tokens.push(Token::Word(word));
                        tokens.push(Token::OpenArguments);
                    }
                    (')', None) => tokens.push(Token::CloseArguments),
                    (')', Some(word)) => {
                        tokens.push(Token::Word(word));
                        tokens.push(Token::CloseArguments);
                    }
                    ('{', None) => tokens.push(Token::OpenObject),
                    ('{', Some(word)) => {
                        tokens.push(Token::Word(word));
                        tokens.push(Token::OpenObject);
                    }
                    ('}', None) => tokens.push(Token::CloseObject),
                    ('}', Some(word)) => {
                        tokens.push(Token::Word(word));
                        tokens.push(Token::CloseObject);
                    }
                    (':', None) => tokens.push(Token::NameSeparator),
                    (':', Some(word)) => {
                        tokens.push(Token::Word(word));
                        tokens.push(Token::NameSeparator);
                    }
                    ('!', None) => tokens.push(Token::NamespaceSeparator),
                    ('!', Some(word)) => {
                        tokens.push(Token::Word(word));
                        tokens.push(Token::NamespaceSeparator);
                    }
                    (' ', word) => return (tokens, word),
                    ('\n', word) => return (tokens, word),
                    (c, None) => return (tokens, Some(String::from(c))),
                    (c, Some(mut word)) => {
                        word.push(c);
                        return (tokens, Some(word.to_string()));
                    }
                };
                (tokens, None)
            });
    tokens
}
