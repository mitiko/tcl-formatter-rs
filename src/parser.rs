use crate::{
    ast::{Ast, Statement},
    lexer::Token,
};

pub struct Parser {}

#[derive(Debug)]
pub struct ParserFail;
type Result<T> = std::result::Result<T, ParserFail>;

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse(self, tokens: &[Token]) -> Result<Ast> {
        Ok(Parser::try_parse(tokens)?.0)
    }

    fn try_parse_if(mut tokens: &[Token]) -> Result<(Ast, usize)> {
        let mut condition_body_clauses = Vec::new();
        let mut consumed = 1; // starts from 1 for the if keyword
        tokens = &tokens[consumed..];

        let condition_tokens = Parser::try_extract_block(tokens)?;
        tokens = &tokens[condition_tokens.len()..];
        consumed += condition_tokens.len();

        let body_tokens = Parser::try_extract_block(tokens)?;
        tokens = &tokens[body_tokens.len()..];
        consumed += body_tokens.len();

        let if_condition = Parser::parse_vec(condition_tokens);
        let (body_if_true, _) = Parser::try_parse(body_tokens)?;
        condition_body_clauses.push((if_condition, body_if_true));

        let maybe_block_if_false = match (tokens.get(0), tokens.get(1)) {
            (Some(Token::KeywordElseIf), Some(Token::LBracket)) => {
                let (ast, consumed_rem) = Parser::try_parse_if(&tokens[1..])?;
                consumed += consumed_rem + 1; // +1 for the elseif keyword
                match ast {
                    Ast::If {
                        condition_body_clauses: elseif_clauses,
                        maybe_block_if_false,
                    } => {
                        condition_body_clauses.extend(elseif_clauses);
                        maybe_block_if_false
                    }
                    _ => return Err(ParserFail),
                }
            }
            (Some(Token::KeywordElse), Some(Token::LBracket)) => {
                let else_body_tokens = Parser::try_extract_block(tokens)?;
                consumed += else_body_tokens.len() + 1; // +1 for the else keyword
                let (block_if_false, _) = Parser::try_parse(else_body_tokens)?;
                Some(Box::new(block_if_false))
            }
            _ => None,
        };

        return Ok((
            Ast::If {
                condition_body_clauses,
                maybe_block_if_false,
            },
            consumed,
        ));
    }

    fn try_parse(mut tokens: &[Token]) -> Result<(Ast, usize)> {
        let mut trees = Vec::new();
        let mut total_consumed = 0;
        while let Some((ast, consumed)) = Parser::try_parse_one(tokens)? {
            dbg!(&ast);
            trees.push(ast);
            total_consumed += consumed;
            tokens = &tokens[consumed..]
        }
        Ok((Ast::Block(trees), total_consumed))
    }

    fn try_parse_one(tokens: &[Token]) -> Result<Option<(Ast, usize)>> {
        let (ast, consumed) = match (tokens.get(0), tokens.get(1)) {
            (Some(Token::KeywordIf), Some(Token::LBracket)) => Parser::try_parse_if(tokens),
            (Some(Token::Hash), Some(Token::Other(comment_text))) => {
                let ast = Ast::Comment(comment_text.to_vec());
                Ok((ast, 2))
            }
            (Some(Token::Newline), _) => Ok((Ast::EmptyLine, 1)),
            (Some(Token::Other(data)), _) => {
                let ast = Ast::Statement(Statement::Other { data: data.to_vec() });
                Ok((ast, 1))
            }
            (None, _) => return Ok(None),
            _ => {
                dbg!(&tokens[0]);
                dbg!(&tokens[1]);
                return Err(ParserFail);
            } // TODO:
        }?;
        Ok(Some((ast, consumed)))
    }

    fn try_extract_block(tokens: &[Token]) -> Result<&[Token]> {
        let mut depth = 1;
        for (idx, token) in tokens.iter().skip(1).enumerate() {
            match token {
                Token::LBracket => depth += 1,
                Token::RBracket => depth -= 1,
                _ => {}
            }
            if depth == 0 {
                return Ok(&tokens[1..idx]);
            }
        }
        Err(ParserFail)
    }

    fn parse_vec(tokens: &[Token]) -> Vec<u8> {
        tokens.into_iter().flat_map(|t| Vec::from(t)).collect()
    }
}
