use crate::lexer::Token;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum AST {
    Empty,
    Numeric(Number),
    String(String),
    List(Vec<AST>),
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Number {
    Integer(i64),
    Decimal(f64),
}

pub fn parser(tokens: &[Token]) -> Option<AST> {
    use Token::*;
    use AST::*;

    match tokens {
        [] => Some(Empty),
        [OpenObject, CloseObject, ..] => Some(AST::List(vec![])),
        [OpenObject, tail @ ..] => {
            if let [inside, outside] = tail
                .splitn(2, |token| matches!(token, CloseObject))
                .collect::<Vec<&[Token]>>()
                .as_slice()
            {
                // parse outside
                Some(List(parse_array_or_object(inside)?))
            } else {
                // Unmatched curly brackets
                None
            }
        }
        tokens => parse(tokens),
    }
}

fn parse_array_or_object(tokens: &[Token]) -> Option<Vec<AST>> {
    tokens
        .split(|token| matches!(token, Token::LineSeparator))
        .map(parser)
        .collect()
}

fn parse(tokens: &[Token]) -> Option<AST> {
    match tokens {
        [] => None,
        [Token::Word(word)] if !word.chars().any(|c| c == '.') => {
            Some(AST::Numeric(Number::Integer(word.parse().ok()?)))
        }
        [Token::Word(word)] => Some(AST::Numeric(Number::Decimal(word.parse().ok()?))),
        [Token::StringLiteralSeparator, Token::Word(word), Token::StringLiteralSeparator] => {
            Some(AST::String(word.to_owned()))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_word_with_integer_as_number_integer() {
        let expected_number = Number::Integer(12);

        assert_eq!(
            parser(&[Token::Word("12".to_string())]),
            Some(AST::Numeric(expected_number))
        )
    }

    #[test]
    fn should_parse_word_with_decimal_as_number_decimal() {
        let expected_number = Number::Decimal(12.24);

        assert_eq!(
            parser(&[Token::Word("12.24".to_string())]),
            Some(AST::Numeric(expected_number))
        )
    }

    #[test]
    fn should_parse_string_litteral_as_string() {
        let expected_ast = AST::String("Toto".to_string());

        assert_eq!(
            parser(&[
                Token::StringLiteralSeparator,
                Token::Word("Toto".to_string()),
                Token::StringLiteralSeparator,
            ]),
            Some(expected_ast)
        )
    }

    #[test]
    fn should_parse_empty_list_objects_as_list() {
        let expected_ast = AST::List(vec![]);

        assert_eq!(
            parser(&[Token::OpenObject, Token::CloseObject]),
            Some(expected_ast)
        )
    }

    #[test]
    fn should_parse_list_with_integer_as_list_of_integer() {
        let expected_ast = AST::List(vec![AST::Numeric(Number::Integer(22))]);

        assert_eq!(
            parser(&[
                Token::OpenObject,
                Token::Word("22".to_string()),
                Token::CloseObject
            ]),
            Some(expected_ast)
        )
    }

    #[test]
    fn should_parse_list_with_decimal_as_list_of_decimal() {
        let expected_ast = AST::List(vec![AST::Numeric(Number::Decimal(22.2))]);

        assert_eq!(
            parser(&[
                Token::OpenObject,
                Token::Word("22.2".to_string()),
                Token::CloseObject
            ]),
            Some(expected_ast)
        )
    }

    #[test]
    fn should_parse_list_with_string_litteral_as_list_of_string_litteral() {
        let expected_ast = AST::List(vec![AST::String("toto".to_string())]);

        assert_eq!(
            parser(&[
                Token::OpenObject,
                Token::StringLiteralSeparator,
                Token::Word("toto".to_string()),
                Token::StringLiteralSeparator,
                Token::CloseObject
            ]),
            Some(expected_ast)
        )
    }

    #[test]
    fn should_parse_list_with_2_string_litteral_as_list_of_2_string_litteral() {
        let expected_ast = AST::List(vec![
            AST::String("toto".to_string()),
            AST::String("tutu".to_string()),
        ]);

        assert_eq!(
            parser(&[
                Token::OpenObject,
                Token::StringLiteralSeparator,
                Token::Word("toto".to_string()),
                Token::StringLiteralSeparator,
                Token::LineSeparator,
                Token::StringLiteralSeparator,
                Token::Word("tutu".to_string()),
                Token::StringLiteralSeparator,
                Token::CloseObject
            ]),
            Some(expected_ast)
        )
    }
}
