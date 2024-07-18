use std::io;
use std::io::Write;

use crate::error::Error;
use crate::scanner::Scanner;

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
        let tokens = Scanner::scan_tokens(buf.clone());
        println!("tokens: {:?}", tokens);
        buf.clear();
    }
}
