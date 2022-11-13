use list::collect_list;

use crate::lexer::Token;

mod list;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum AST {
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

pub fn parser(tokens: &[Token]) -> Vec<Option<AST>> {
    let mut asts = Vec::new();

    parser_initialised(tokens, &mut asts);
    asts
}

fn parser_initialised(tokens: &[Token], asts: &mut Vec<Option<AST>>) {
    match tokens {
        [] => {}
        // TODO: Line comment Pattern,
        tokens => {
            let (ast, tokens) = parse_expression(tokens);

            asts.push(ast);
            parser_initialised(tokens, asts)
        }
    }
}

fn parse_expression(tokens: &[Token]) -> (Option<AST>, &[Token]) {
    match tokens {
        [] => unreachable!("parser_initialised already filtered the [] pattern."),
        [Token::Word(word), tail @ ..] if word.chars().any(|c| c == '.') => (
            word.parse().ok().map(Number::Decimal).map(AST::Numeric),
            tail,
        ),
        [Token::Word(word), tail @ ..] => (
            word.parse().ok().map(Number::Integer).map(AST::Numeric),
            tail,
        ),
        [Token::StringLiteralSeparator, Token::Word(word), Token::StringLiteralSeparator, tail @ ..] => {
            (Some(AST::String(word.to_owned())), tail)
        }
        //[Token::OpenObject, _htail @ Token::Word(_), _tail @ ..] => todo!("Parse object"),
        [Token::OpenObject, tail @ ..] => collect_list(tail),
        t => panic!("{t:?}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod numbers {
        use super::*;

        #[test]
        fn should_parse_word_with_integer_as_number_integer() {
            let expected_number = Number::Integer(12);

            assert_eq!(
                parser(&[Token::Word("12".to_string())]),
                vec![Some(AST::Numeric(expected_number))]
            )
        }

        #[test]
        fn should_parse_word_with_decimal_as_number_decimal() {
            let expected_number = Number::Decimal(12.24);

            assert_eq!(
                parser(&[Token::Word("12.24".to_string())]),
                vec![Some(AST::Numeric(expected_number))]
            )
        }
    }

    mod strings {
        use super::*;

        #[test]
        fn should_parse_string_litteral_as_string() {
            let expected_ast = AST::String("Toto".to_string());

            assert_eq!(
                parser(&[
                    Token::StringLiteralSeparator,
                    Token::Word("Toto".to_string()),
                    Token::StringLiteralSeparator,
                ]),
                vec![Some(expected_ast)]
            )
        }
    }

    mod lists {
        use super::*;

        #[test]
        fn should_parse_empty_list_objects_as_list() {
            let expected_ast = AST::List(vec![]);

            assert_eq!(
                parser(&[Token::OpenObject, Token::CloseObject]),
                vec![Some(expected_ast)]
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
                vec![Some(expected_ast)]
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
                vec![Some(expected_ast)]
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
                vec![Some(expected_ast)]
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
                vec![Some(expected_ast)]
            )
        }

        #[test]
        fn should_parse_list_of_diverse_types_as_list_of_variant() {
            let expected_ast = AST::List(vec![
                AST::String("toto".to_string()),
                AST::Numeric(Number::Integer(12)),
                AST::Numeric(Number::Decimal(12.2)),
            ]);

            assert_eq!(
                parser(&[
                    Token::OpenObject,
                    Token::StringLiteralSeparator,
                    Token::Word("toto".to_string()),
                    Token::StringLiteralSeparator,
                    Token::LineSeparator,
                    Token::Word("12".to_string()),
                    Token::LineSeparator,
                    Token::Word("12.2".to_string()),
                    Token::CloseObject
                ]),
                vec![Some(expected_ast)]
            )
        }

        #[test]
        fn should_parse_list_of_list_as_nested_lists() {
            let extpected_ast = AST::List(vec![AST::List(vec![])]);

            assert_eq!(
                parser(&[
                    Token::OpenObject,
                    Token::OpenObject,
                    Token::CloseObject,
                    Token::CloseObject
                ]),
                vec![Some(extpected_ast)]
            )
        }
    }
}
