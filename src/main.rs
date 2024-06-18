use std::io;

mod ast;
mod formatter;
mod lexer;
mod parser;

use formatter::*;
use lexer::*;
use parser::*;

fn main() -> io::Result<()> {
    let path =
        "/home/drusev@efellows.bg/Documents/Projects/a1-gtp-proxy/src/GTP-C-clientAcceptV5.tcl";
    let buf = std::fs::read(path)?;
    let tokens = Lexer::new().lex(buf);
    let ast = Parser::new().parse(&tokens).expect("Failed to parse");
    let buf = Formatter::new().format(ast); // cursed interface
    std::fs::write(path, buf)?;
    Ok(())
}
