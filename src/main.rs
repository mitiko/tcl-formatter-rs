use std::io;

mod ast;
mod formatter;
mod lexer;
mod parser;

use formatter::*;
use lexer::*;
use parser::*;

fn main() -> io::Result<()> {
    for path in [
        "../a1-gtp-proxy/src/GTP-C-INIT_V5.tcl",
        "../a1-gtp-proxy/src/GTP-C-clientAcceptV5.tcl",
        "../a1-gtp-proxy/src/GTP-C-clientAcceptv3.tcl",
        "../a1-gtp-proxy/src/GTP-C-clientAcceptv31.tcl",
        "../a1-gtp-proxy/src/GTP-C-clientEgressV5.tcl",
        "../a1-gtp-proxy/src/GTP-C-clientEgressv3.tcl",
        "../a1-gtp-proxy/src/GTP-C-clientIngressV5.tcl",
        "../a1-gtp-proxy/src/GTP-C-clientIngressv3.tcl",
        "../a1-gtp-proxy/src/GTP-C-clientIngressv31.tcl",
        "../a1-gtp-proxy/src/GTP-C-variables_V5.tcl",
        "../a1-gtp-proxy/src/GTP-U_v4.tcl",
        "../a1-gtp-proxy/src/GTP-Uv3.tcl",
        "../a1-gtp-proxy/src/GTP-Uv31.tcl",
        "../a1-gtp-proxy/src/lib_GTPutil_V4.tcl",
        "../a1-gtp-proxy/src/lib_GTPutil_V5.tcl",
        "../a1-gtp-proxy/src/lib_GTPutilv3.tcl",
        "../a1-gtp-proxy/src/lib_LogUtil.tcl",
    ] {
        println!("formatting {path}");
        let buf = std::fs::read(path)?;
        let tokens = Lexer::new().lex(buf).expect("Failed to lex");
        let ast = Parser::new().parse(&tokens).expect("Failed to parse");
        let buf = Formatter::new().format(ast); // cursed interface
        std::fs::write(path, buf)?;
    }
    Ok(())
}
