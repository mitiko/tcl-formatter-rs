use std::iter;

use crate::ast::{Ast, Statement};

pub fn format(ast: Ast) -> Vec<u8> {
    format_with_indent(0, ast)
}

fn format_with_indent(level: usize, ast: Ast) -> Vec<u8> {
    let mut buf = Vec::new();
    // TODO: handle newlines

    match ast {
        Ast::Block(statements) => {
            for s in statements {
                buf.extend_from_slice(&format_with_indent(level, s));
            }
        }
        Ast::Comment(data) => {
            buf.extend_from_slice(&indent(level));
            buf.extend_from_slice(b"# ");
            buf.extend_from_slice(&data);
            buf.push(b'\n');
        }
        Ast::Procedure {
            name,
            parameters,
            body,
        } => {
            buf.extend_from_slice(&indent(level));
            buf.extend_from_slice(b"proc ");
            buf.extend_from_slice(&name);
            buf.extend_from_slice(b" {");
            for p in parameters {
                buf.push(b' ');
                buf.extend_from_slice(&p);
            }
            buf.extend_from_slice(b" } {\n");
            buf.extend_from_slice(&format_with_indent(level + 1, *body));
            buf.extend_from_slice(&indent(level));
            buf.extend_from_slice(b"}\n");
        }
        Ast::If { condition, body } => {
            buf.extend_from_slice(&indent(level));
            buf.extend_from_slice(b"if { ");
            buf.extend_from_slice(&condition);
            buf.extend_from_slice(b" } {\n");
            buf.extend_from_slice(&format_with_indent(level + 1, *body));
            buf.extend_from_slice(&indent(level));
            buf.extend_from_slice(b"}\n");
        }
        Ast::IfElse {
            condition,
            block_if_true,
            block_if_false,
        } => {
            buf.extend_from_slice(&indent(level));
            buf.extend_from_slice(b"if { ");
            buf.extend_from_slice(&condition);
            buf.extend_from_slice(b" } {\n");
            buf.extend_from_slice(&format_with_indent(level + 1, *block_if_true));
            buf.extend_from_slice(&indent(level));
            buf.extend_from_slice(b"}\n");
            buf.extend_from_slice(&indent(level));
            buf.extend_from_slice(b"else {\n");
            buf.extend_from_slice(&format_with_indent(level + 1, *block_if_false));
            buf.extend_from_slice(&indent(level));
            buf.extend_from_slice(b"}\n");
        }
        Ast::IfElseIf {
            condition_block_vec,
            block_if_false,
        } => {
            for (idx, (condition, block)) in condition_block_vec.into_iter().enumerate() {
                buf.extend_from_slice(&indent(level));
                if idx == 0 {
                    buf.extend_from_slice(b"if { ");
                } else {
                    buf.extend_from_slice(b"elseif { ");
                }
                buf.extend_from_slice(&condition);
                buf.extend_from_slice(b" } {\n");
                buf.extend_from_slice(&format_with_indent(level + 1, block));
                buf.extend_from_slice(&indent(level));
                buf.extend_from_slice(b"}\n");
            }
            buf.extend_from_slice(b"else {\n");
            buf.extend_from_slice(&format_with_indent(level + 1, *block_if_false));
            buf.extend_from_slice(&indent(level));
            buf.extend_from_slice(b"}\n");
        }
        Ast::Switch {
            condition,
            value_block_or_fallthrough_vec,
        } => {
            // TODO: sort conditions of fallthrough blocks
            buf.extend_from_slice(&indent(level));
            buf.extend_from_slice(b"switch ");
            buf.extend_from_slice(&condition);
            buf.extend_from_slice(b" {\n");
            for (value, block_or_fallthrough) in value_block_or_fallthrough_vec {
                buf.extend_from_slice(&indent(level + 1));
                buf.extend_from_slice(&value);
                match block_or_fallthrough {
                    Some(block) => {
                        buf.extend_from_slice(b" {\n");
                        buf.extend_from_slice(&format_with_indent(level + 2, block));
                        buf.extend_from_slice(&indent(level + 1));
                        buf.extend_from_slice(b"}\n");
                    }
                    None => {
                        buf.extend_from_slice(b" -\n");
                    }
                }
            }
            buf.extend_from_slice(&indent(level));
            buf.extend_from_slice(b"}\n");
        }
        Ast::Statement(s) => {
            buf.extend_from_slice(&indent(level));
            buf.extend_from_slice(&fmt_statement(s));
        }
    }

    buf
}

fn indent(level: usize) -> Vec<u8> {
    iter::repeat(b"    ")
        .take(level)
        .fold(Vec::new(), |mut acc, e| {
            acc.extend_from_slice(e);
            acc
        })
}

fn fmt_statement(s: Statement) -> Vec<u8> {
    let (keyword, v1, v2): (&[u8], _, _) = match s {
        Statement::Set { identifier, value } => (b"set", Some(identifier), Some(value)),
        Statement::Log { bucket, value } => (b"log", Some(bucket), Some(value)),
        Statement::Snat { ip_address, port } => (b"snat", Some(ip_address), Some(port)),
        Statement::Node { ip_address, port } => (b"node", Some(ip_address), Some(port)),
        Statement::Pool { identifier } => (b"pool", Some(identifier), None),
        Statement::SnatPool { identifier } => (b"snatpool", Some(identifier), None),
        Statement::Return { value } => (b"return", value, None),
    };
    let mut buf = Vec::new();
    buf.extend_from_slice(keyword);
    match (v1, v2) {
        (Some(v1), Some(v2)) => {
            buf.push(b' ');
            buf.extend_from_slice(&v1);
            buf.push(b' ');
            buf.extend_from_slice(&v2);
        }
        (Some(v1), None) => {
            buf.push(b' ');
            buf.extend_from_slice(&v1);
        }
        (None, None) => todo!(),
        _ => unreachable!(),
    }
    buf.push(b'\n');
    buf
}
