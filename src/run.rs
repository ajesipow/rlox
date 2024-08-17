use std::io;
use std::io::Write;

use itertools::Itertools;

use crate::error::Error;
use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;

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
        match run(&buf) {
            Ok(o) => {
                println!("{o}");
            }
            Err(e) => {
                println!("{e}");
            }
        }
        buf.clear();
    }
}

fn run(buf: &str) -> Result<String, Error> {
    let tokens = Lexer::lex(buf);
    let mut parser = Parser::new(tokens.into_iter().flatten().collect_vec());
    let ast = parser.parse()?;
    let output = Interpreter::interpret(ast)?;
    Ok(output)
}
