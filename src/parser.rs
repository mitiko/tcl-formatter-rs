use crate::{ast::Ast, position::Position};

pub struct Parser {
    buf: Vec<u8>,
    pos: Position,
}

impl Parser {
    pub fn new(buf: Vec<u8>) -> Self {
        Self {
            buf,
            pos: Position::default(),
        }
    }

    pub fn parse(&mut self) -> Ast {
        todo!()
    }

    pub fn parse_shallow(&self) -> Vec<Ast> {
        todo!()
    }
}
