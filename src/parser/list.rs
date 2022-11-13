use crate::lexer::Token;
use crate::parser::{parse_expression, AST};

pub fn collect_list(tokens: &[Token]) -> (Option<AST>, &[Token]) {
    let mut asts = vec![];
    let left_tokens = collect_list_initialised(&mut asts, tokens);

    (
        asts.into_iter().collect::<Option<Vec<_>>>().map(AST::List),
        left_tokens,
    )
}

fn collect_list_initialised<'a, 'b>(
    asts: &'a mut Vec<Option<AST>>,
    tokens: &'b [Token],
) -> &'b [Token] {
    match tokens {
        [] => {
            asts.push(None);
            &[]
        }
        [Token::CloseObject, tail @ ..] => tail,
        tokens => {
            let (ast, left_tokens) = parse_expression(tokens);

            asts.push(ast);
            collect_list_next_elements(asts, left_tokens)
        }
    }
}

fn collect_list_next_elements<'a, 'b>(
    asts: &'a mut Vec<Option<AST>>,
    tokens: &'b [Token],
) -> &'b [Token] {
    match tokens {
        [] => {
            asts.push(None);
            &[]
        }
        [Token::CloseObject, tail @ ..] => tail,
        [Token::LineSeparator, tail @ ..] => {
            let (ast, left_tokens) = parse_expression(tail);

            asts.push(ast);
            collect_list_next_elements(asts, left_tokens)
        }
        _ => {
            asts.push(None);
            &[]
        }
    }
}
