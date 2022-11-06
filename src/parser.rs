use crate::lexer::Token;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum AST {
    Numeric(Number),
    String(String),
    Array(Vec<AST>),
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Number {
    Integer(i64),
    Decimal(f64),
}

pub fn parser(tokens: Vec<Token>) -> Option<AST> {
    match tokens.as_slice() {
        [] => None,
        [Token::OpenObject, tail @ ..] => Some(AST::Array(parse_array_or_object(tail)?)),
        _ => None,
    }
}

fn parse_array_or_object(tokens: &[Token]) -> Option<Vec<AST>> {
    match tokens {
        [_token, Token::LineSeparator, _tail @ ..] => todo!(),
        [_token, Token::CloseObject, _tail @ ..] => todo!(),
        [Token::CloseObject, _tail @ ..] => todo!(),
        _ => todo!(),
    }
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
            parser(vec![Token::Word("12".to_string())]),
            Some(AST::Numeric(expected_number))
        )
    }

    #[test]
    fn should_parse_word_with_decimal_as_number_decimal() {
        let expected_number = Number::Decimal(12.24);

        assert_eq!(
            parser(vec![Token::Word("12.24".to_string())]),
            Some(AST::Numeric(expected_number))
        )
    }

    #[test]
    fn should_parse_string_litteral_as_string() {
        let expected_ast = AST::String("Toto".to_string());

        assert_eq!(
            parser(vec![
                Token::StringLiteralSeparator,
                Token::Word("Toto".to_string()),
                Token::StringLiteralSeparator,
            ]),
            Some(expected_ast)
        )
    }

    #[test]
    fn should_parse_list_objects_as_lists() {
        let expected_ast = AST::Array(vec![]);

        assert_eq!(
            parser(vec![Token::OpenObject, Token::CloseObject]),
            Some(expected_ast)
        )
    }
}
