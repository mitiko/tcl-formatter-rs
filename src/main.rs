use std::io;

mod ast;
mod formatter;
mod parser;
mod position;

use formatter::*;
use parser::Parser;

fn main() -> io::Result<()> {
    let path =
        "/home/drusev@efellows.bg/Documents/Projects/a1-gtp-proxy/src/GTP-C-clientAcceptV5.tcl";
    let buf = std::fs::read(path)?;
    let ast = Parser::new(buf).parse();
    let mut fmt = Formatter::new();
    fmt.run(ast);
    std::fs::write(path, fmt.output())?;
    Ok(())
}
