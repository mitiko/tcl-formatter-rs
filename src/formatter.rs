use std::iter;

use crate::ast::{Ast, Statement};

pub struct Formatter {
    depth: usize,
    consecutive_empty_lines: usize,
    buf: Vec<u8>,
}

impl Formatter {
    pub fn new() -> Self {
        Self {
            depth: 0,
            consecutive_empty_lines: 0,
            buf: Vec::new(),
        }
    }

    pub fn format(mut self, ast: Ast) -> Vec<u8> {
        self.run(ast);
        self.buf
    }

    fn run(&mut self, ast: Ast) {
        self.consecutive_empty_lines = match ast {
            Ast::EmptyLine => self.consecutive_empty_lines + 1,
            _ => 0,
        };

        match ast {
            Ast::Block(trees) => {
                for tree in trees {
                    self.run(tree);
                }
            }
            Ast::Comment(data) => {
                self.indent();
                self.write(b"# ");
                self.write(&data);
                self.newline();
            }
            Ast::Procedure {
                name,
                parameters,
                body,
            } => {
                self.indent();
                self.write(b"proc ");
                self.write(&name);
                self.write(b" {");
                for p in parameters {
                    self.write(b" ");
                    self.write(&p);
                }
                self.writeline(b" } {");
                self.run_nested(*body);
                self.close_block();
            }
            Ast::If {
                condition_body_clauses: condition_block_vec,
                maybe_block_if_false,
            } => {
                for (idx, (condition, block)) in condition_block_vec.into_iter().enumerate() {
                    self.indent();
                    if idx == 0 {
                        self.write(b"if { ");
                    } else {
                        self.write(b"elseif { ");
                    }
                    self.write(&condition);
                    self.writeline(b" } {");
                    self.run_nested(block);
                    self.close_block();
                }
                if let Some(block_if_false) = maybe_block_if_false {
                    self.indent();
                    self.writeline(b"else {");
                    self.run_nested(*block_if_false);
                    self.close_block();
                }
            }
            Ast::Switch {
                condition,
                value_block_or_fallthrough_vec,
            } => {
                // TODO: sort conditions of fallthrough blocks
                self.indent();
                self.write(b"switch ");
                self.write(&condition);
                self.writeline(b" {");

                self.depth += 1;
                for (value, block_or_fallthrough) in value_block_or_fallthrough_vec {
                    self.indent();
                    self.write(&value);
                    match block_or_fallthrough {
                        Some(block) => {
                            self.writeline(b" {");
                            self.run_nested(block);
                            self.close_block();
                        }
                        None => {
                            self.writeline(b" -");
                        }
                    }
                }
                self.depth -= 1;
                self.close_block();
            }
            Ast::Statement(s) => {
                self.indent();
                self.write_statement(s);
            }
            Ast::EmptyLine => {
                if self.consecutive_empty_lines <= 2 {
                    self.newline();
                }
            },
            Ast::When { event_name, body } => {
                self.indent();
                self.write(b"when ");
                self.write(&event_name);
                self.writeline(b" {");
                self.run_nested(*body);
                self.close_block();
            }
        }
    }

    fn run_nested(&mut self, ast: Ast) {
        self.depth += 1;
        self.run(ast);
        self.depth -= 1;
    }

    fn write_statement(&mut self, s: Statement) {
        let (keyword, v1, v2) = match s {
            Statement::Set { identifier, value } => (b"set".to_vec(), Some(identifier), Some(value)),
            Statement::Log { bucket, value } => (b"log".to_vec(), Some(bucket), Some(value)),
            Statement::Snat { ip_address, port } => (b"snat".to_vec(), Some(ip_address), Some(port)),
            Statement::Node { ip_address, port } => (b"node".to_vec(), Some(ip_address), Some(port)),
            Statement::Pool { identifier } => (b"pool".to_vec(), Some(identifier), None),
            Statement::SnatPool { identifier } => (b"snatpool".to_vec(), Some(identifier), None),
            Statement::Return { value } => (b"return".to_vec(), value, None),
            Statement::Other { data } => (data, None, None),
        };
        self.write(&keyword);
        match (v1, v2) {
            (Some(v1), Some(v2)) => {
                self.write(b" ");
                self.write(&v1);
                self.write(b" ");
                self.write(&v2);
            }
            (Some(v1), None) => {
                self.write(b" ");
                self.write(&v1);
            }
            (None, None) => {},
            _ => unreachable!(),
        }
        self.newline();
    }

    fn write(&mut self, slice: &[u8]) {
        self.buf.extend_from_slice(slice);
    }

    fn writeline(&mut self, slice: &[u8]) {
        self.write(slice);
        self.newline();
    }

    fn newline(&mut self) {
        self.buf.push(b'\n');
    }

    fn close_block(&mut self) {
        self.indent();
        self.write(b"}\n");
    }

    fn indent(&mut self) {
        let data = iter::repeat(b"    ")
            .take(self.depth)
            .fold(Vec::new(), |mut acc, e| {
                acc.extend_from_slice(e);
                acc
            });
        self.buf.extend_from_slice(&data);
    }
}
