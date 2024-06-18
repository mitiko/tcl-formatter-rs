use crate::{ast::Ast, lexer::Token};

pub struct Parser {}

#[derive(Debug)]
pub struct ParserFail;
type Result<T> = std::result::Result<T, ParserFail>;

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse(self, tokens: &[Token]) -> Result<Ast> {
        Parser::try_parse(tokens)
    }

    fn try_parse_if(mut tokens: &[Token]) -> Result<Ast> {
        let mut condition_body_clauses = Vec::new();

        let condition_tokens = Parser::try_extract_block(tokens)?;
        tokens = &tokens[condition_tokens.len()..];

        let body_tokens = Parser::try_extract_block(tokens)?;
        tokens = &tokens[body_tokens.len()..];

        let if_condition = Parser::parse_vec(condition_tokens);
        let body_if_true = Parser::try_parse(body_tokens)?;
        condition_body_clauses.push((if_condition, body_if_true));

        let maybe_block_if_false = match (tokens.get(0), tokens.get(1)) {
            (Some(Token::KeywordElseIf), Some(Token::LBracket)) => {
                match Parser::try_parse_if(&tokens[1..])? {
                    Ast::If {
                        condition_block_vec,
                        maybe_block_if_false,
                    } => {
                        condition_body_clauses.extend(condition_block_vec);
                        maybe_block_if_false
                    }
                    _ => return Err(ParserFail),
                }
            }
            (Some(Token::KeywordElse), Some(Token::LBracket)) => {
                let else_body_tokens = Parser::try_extract_block(tokens)?;
                let block_if_false = Parser::try_parse(else_body_tokens)?;
                Some(Box::new(block_if_false))
            }
            _ => None,
        };

        return Ok(Ast::If {
            condition_block_vec: condition_body_clauses,
            maybe_block_if_false,
        });
    }

    fn try_parse(tokens: &[Token]) -> Result<Ast> {
        match (tokens.get(0), tokens.get(1)) {
            (Some(Token::KeywordIf), Some(Token::LBracket)) => Parser::try_parse_if(&tokens[1..]),
            _ => return Err(ParserFail),
            // TODO:
        }
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
