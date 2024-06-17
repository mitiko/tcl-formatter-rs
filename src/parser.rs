use crate::{ast::Ast, lexer::Token};

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse(mut self, tokens: Vec<Token>) -> Ast {
        self.run(&tokens)
    }

    fn run(&mut self, tokens: &[Token]) -> Ast {
        todo!()
    }
}
