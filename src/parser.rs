use crate::lexer::Token;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
enum Number {
    Integer(i64),
    Decimal(f64),
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum AST {
    Numeric(Number),
}

pub fn parser(tokens: Vec<Token>) -> Option<AST> {
    match tokens.as_slice() {
        [] => None,
        [Token::Word(word)] if word.chars().find(|&c| c == '.').is_none() => {
            Some(AST::Numeric(Number::Integer(word.parse().ok()?)))
        }
        [Token::Word(word)] => Some(AST::Numeric(Number::Decimal(word.parse().ok()?))),
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
}
