use std::io;

mod ast;
mod formatter;
mod lexer;
mod parser;

use formatter::*;
use lexer::*;
use parser::*;

fn main() -> io::Result<()> {
    let path = "../a1-gtp-proxy/src/GTP-C-clientAcceptV5.tcl";
    let buf = std::fs::read(path)?;
    let tokens = Lexer::new().lex(buf).expect("Failed to lex");
    // print all tokens
    // for token in tokens.iter() {
    //     println!("{:?}", token);
    // }
    // print comments only
    // for (t1, t2) in tokens.iter().zip(tokens.iter().skip(1)) {
    //     if let Token::Other(_) = t2 {
    //         println!("{:?}{:?}", t1, t2);
    //     }
    // }
    // debug bracket balance
    // let cnt_lbracket = tokens.iter().filter(|&x| matches!(x, Token::LCurlyBracket)).count();
    // let cnt_rbracket = tokens.iter().filter(|&x| matches!(x, Token::RCurlyBracket)).count();
    // dbg!(cnt_lbracket);
    // dbg!(cnt_rbracket);
    let ast = Parser::new().parse(&tokens).expect("Failed to parse");
    let buf = Formatter::new().format(ast); // cursed interface
    std::fs::write(path, buf)?;
    Ok(())
}
