use std::io;
use std::io::Write;

use crate::error::Error;
use crate::scanner::Lexer;

pub fn run_prompt() -> Result<(), Error> {
    let mut buf = String::new();
    loop {
        let mut lock = io::stdout().lock();
        lock.write_all(b"> ")?;
        lock.flush()?;
        drop(lock);
        let bytes_read = io::stdin().read_line(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        let tokens = Lexer::lex(&buf);
        println!("tokens: {:?}", tokens);
        buf.clear();
    }
}
