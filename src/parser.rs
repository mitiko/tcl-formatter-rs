use crate::{
    ast::{Ast, Statement},
    lexer::Token,
};

pub struct Parser {}

#[derive(Debug)]
pub enum ParserFail {
    ElseIfBlock,
    BracketMismatch,
    UnknownAST, // no tokens matched an AST block
    Other,      // TODO: remove this
}
type Result<T> = std::result::Result<T, ParserFail>;

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse(self, tokens: &[Token]) -> Result<Ast> {
        Ok(Parser::try_parse(tokens)?.0)
    }

    fn try_parse_if(mut tokens: &[Token]) -> Result<(Ast, usize)> {
        println!("parsing if");
        let mut condition_body_clauses = Vec::new();
        let mut consumed = 1; // starts from 1 for the if keyword
        tokens = &tokens[consumed..];

        let condition_tokens = Parser::try_extract_block(tokens)?;
        tokens = &tokens[condition_tokens.len() + 2..];
        consumed += condition_tokens.len() + 2;

        let body_tokens = Parser::try_extract_block(tokens)?;
        tokens = &tokens[body_tokens.len() + 2..];
        consumed += body_tokens.len() + 2;

        let if_condition = Parser::parse_vec(condition_tokens);
        let (body_if_true, _) = Parser::try_parse(body_tokens)?;
        condition_body_clauses.push((if_condition, body_if_true));

        let maybe_block_if_false = match (tokens.get(0), tokens.get(1)) {
            (Some(Token::KeywordElseIf), Some(Token::LCurlyBracket)) => {
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
                    _ => return Err(ParserFail::ElseIfBlock),
                }
            }
            (Some(Token::KeywordElse), Some(Token::LCurlyBracket)) => {
                let else_body_tokens = Parser::try_extract_block(tokens)?;
                consumed += else_body_tokens.len() + 1 + 2; // +1 for the else keyword, +2 for brackets
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

    fn try_parse_when(mut tokens: &[Token]) -> Result<(Ast, usize)> {
        println!("parsing when");
        let Token::Identifier(event_name) = &tokens[1] else {
            unreachable!();
        };

        let mut consumed = 2; // starts from 2 for the when keyword & the event name
        tokens = &tokens[consumed..];

        let body_tokens = Parser::try_extract_block(tokens)?;
        consumed += body_tokens.len() + 2;

        let (body_if_true, _) = Parser::try_parse(body_tokens)?;

        return Ok((
            Ast::When {
                event_name: event_name.to_vec(),
                body: Box::new(body_if_true),
            },
            consumed,
        ));
    }

    fn try_parse_set(tokens: &[Token]) -> Result<(Ast, usize)> {
        println!("parsing set");
        let Token::Identifier(data) = &tokens[1] else {
            unreachable!();
        };
        let mut consumed = 2; // starts from 2 for the set keyword & the identifier

        let mut space_iterator = data.split(|&x| x == b' ');
        let identifier = space_iterator.next().ok_or(ParserFail::Other)?.to_vec();
        let mut value = space_iterator
            .next()
            .map(|x| x.to_vec())
            .unwrap_or(Vec::new());

        for token in &tokens[2..] {
            consumed += 1;
            value.extend(Vec::from(token));
            if let Token::Newline = token {
                break;
            }
        }

        return Ok((
            Ast::Statement(Statement::Set { identifier, value }),
            consumed,
        ));
    }

    fn try_parse_log(tokens: &[Token]) -> Result<(Ast, usize)> {
        println!("parsing log");
        let Token::Identifier(data) = &tokens[1] else {
            unreachable!();
        };
        let mut consumed = 3; // starts from 3 for the log keyword, the bucket, the quote
        let bucket = data.to_vec();
        let mut value = Vec::new();

        for token in &tokens[3..] {
            consumed += 1;
            value.extend(Vec::from(token));
            // TODO: error on newline
            if let Token::Quote = token {
                break;
            }
        }

        return Ok((Ast::Statement(Statement::Log { bucket, value }), consumed));
    }

    fn try_parse(mut tokens: &[Token]) -> Result<(Ast, usize)> {
        println!("-> recursive call to try_parse");
        let mut trees = Vec::new();
        let mut total_consumed = 0;
        loop {
            let (ast, consumed) = match Parser::try_parse_one(tokens)? {
                (None, 0) => break,
                (None, 1) => {
                    tokens = &tokens[1..];
                    total_consumed += 1;
                    continue;
                }
                (Some(ast), consumed) => (ast, consumed),
                _ => unreachable!(),
            };
            tokens = &tokens[consumed..];
            total_consumed += consumed;
            dbg!(&ast);
            trees.push(ast);
        }
        Ok((Ast::Block(trees), total_consumed))
    }

    fn try_parse_one(tokens: &[Token]) -> Result<(Option<Ast>, usize)> {
        let (ast, consumed) = match (tokens.get(0), tokens.get(1), tokens.get(2)) {
            (Some(Token::Hash), Some(Token::Other(comment_text)), Some(Token::Newline)) => {
                // comment
                let ast = Ast::Comment(comment_text.to_vec());
                Ok((ast, 2))
            }
            (Some(Token::KeywordIf), Some(Token::LCurlyBracket), ..) => {
                Parser::try_parse_if(tokens)
            }
            (Some(Token::KeywordWhen), Some(Token::Identifier(_)), Some(Token::LCurlyBracket)) => {
                Parser::try_parse_when(tokens)
            }
            (Some(Token::KeywordSet), Some(Token::Identifier(_)), ..) => {
                Parser::try_parse_set(tokens)
            }
            (Some(Token::KeywordLog), Some(Token::Identifier(_)), Some(Token::Quote)) => {
                Parser::try_parse_log(tokens)
            }

            (Some(Token::Newline), Some(Token::Newline), ..) => Ok((Ast::EmptyLine, 2)),
            (Some(Token::Newline), Some(_), ..) => return Ok((None, 1)), // eat newline
            (None, ..) => return Ok((None, 0)),
            _ => {
                dbg!(&tokens[0]);
                dbg!(&tokens[1]);
                dbg!(&tokens[2]);
                return Err(ParserFail::UnknownAST);
            } // TODO:
        }?;
        Ok((Some(ast), consumed))
    }

    fn try_extract_block(tokens: &[Token]) -> Result<&[Token]> {
        let mut depth = 0;
        for (idx, token) in tokens.iter().enumerate() {
            match token {
                Token::LCurlyBracket => depth += 1,
                Token::RCurlyBracket => depth -= 1,
                _ => {}
            }
            if depth == 0 {
                return Ok(&tokens[1..idx]);
            }
        }
        Err(ParserFail::BracketMismatch)
    }

    fn parse_vec(tokens: &[Token]) -> Vec<u8> {
        tokens.into_iter().flat_map(|t| Vec::from(t)).collect()
    }
}
