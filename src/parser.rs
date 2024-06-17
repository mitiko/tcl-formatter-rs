use crate::{ast::Ast, lexer::Token};

pub struct Parser {}

struct ParserFail;
type Result<T> = std::result::Result<T, ParserFail>;

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse(mut self, tokens: &[Token]) -> Ast {
        self.run(&tokens)
    }

    fn try_parse_if_condition(mut tokens: &[Token]) -> Result<Ast> {
        match (tokens.get(0), tokens.get(1)) {
            (Some(Token::KeywordIf), Some(Token::LBracket)) => {}
            _ => return Err(ParserFail),
        }
        tokens = &tokens[1..];

        let condition_tokens = Parser::try_parse_block(tokens)?;
        tokens = &tokens[condition_tokens.len()..];

        let condition = Parser::try_parse(condition_tokens)?;

        let body_tokens = Parser::try_parse_block(tokens)?;
        let body = Parser::try_parse(body_tokens)?;

        // TODO: condition to Vec<u8>
        return Ok(Ast::If { condition: Vec::new(), body: Box::new(body) });
    }

    fn try_parse(tokens: &[Token]) -> Result<Ast> {
        todo!()
    }

    fn try_parse_block(tokens: &[Token]) -> Result<&[Token]> {
        match tokens.get(0) {
            Some(Token::LBracket) => {}
            _ => return Err(ParserFail),
        }
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

    fn run(&mut self, tokens: &[Token]) -> Ast {
        let res = Vec::new();
        let mut groups = Parser::parse_groups(tokens).into_iter();

        let Some(group) = groups.next() else {
            return Ast::Block(Vec::new());
        };
        match group {
            TokenGroup::Single(Token::KeywordIf) => {}
            TokenGroup::Single(Token::KeywordNode) => {}
            _ => {}
        }
        Ast::Block(res)
    }
}
