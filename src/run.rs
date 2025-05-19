use std::io;
use std::io::Write;
use std::rc::Rc;

use itertools::Itertools;

use crate::error::Error;
use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;

pub fn run_repl() -> Result<(), Error> {
    let mut input = vec![];
    let mut interpreter = Interpreter::new();
    loop {
        let mut lock = io::stdout().lock();
        lock.write_all(b"> ")?;
        lock.flush()?;
        drop(lock);
        let mut buf = String::new();
        let bytes_read = io::stdin().read_line(&mut buf)?;
        let rc: Rc<str> = buf.into();
        input.push(rc.clone());

        if bytes_read == 0 {
            return Ok(());
        }

        if let Err(e) = run(rc, &mut interpreter) {
            println!("{e}");
        }
    }
}

fn run(
    buf: Rc<str>,
    interpreter: &mut Interpreter,
) -> Result<(), Error> {
    let tokens = Lexer::lex(buf);
    let mut parser = Parser::new(tokens.into_iter().flatten().collect_vec());
    let ast = parser.parse()?;
    interpreter.interpret(ast)?;
    Ok(())
}
