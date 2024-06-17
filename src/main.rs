use std::io;

mod fmt;
use fmt::*;

fn main() -> io::Result<()> {
    let path = "/home/drusev@efellows.bg/Documents/Projects/a1-gtp-proxy/src/GTP-C-clientAcceptV5.tcl";
    let buf = std::fs::read(path)?;
    let formatter = Formatter::new(buf);
    let buf = formatter.run();
    std::fs::write(path, buf)?;
    Ok(())
}
