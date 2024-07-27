use std::path::PathBuf;

use clap::Parser;
use rlox::ast::{Expr, PrettyPrinter, Visitor};
use rlox::error::PublicError;
use rlox::read_source_file;
use rlox::run_prompt;
use rlox::token::{Token, TokenKind};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // The path of a rlox source file to interpret
    #[arg(short, long)]
    file: Option<PathBuf>,
}

fn main() -> Result<(), PublicError> {
    let ast = Expr::Binary {
        left: Box::new(Expr::Unary { operator: Token::new(TokenKind::Minus, Some("-".to_string()), 1), right: Box::new(Expr::Literal("123".to_string())) }),
        operator: Token::new(TokenKind::Star, Some("*".to_string()), 1),
        right: Box::new(Expr::Grouping { expression: Box::new(Expr::Literal("45.67".to_string())) }),
    };
    
    let output = PrettyPrinter{}.visit_expr(&ast);
    println!("{output}");
    
    
    // let args = Args::parse();
    //
    // if let Some(file_path) = args.file {
    //     // TODO put this behind struct
    //     let raw_source = read_source_file(&file_path)?;
    // } else {
    //     // TODO put this behind struct
    //     run_prompt()?;
    // }
    // println!("Hello, world!");
    Ok(())
}
