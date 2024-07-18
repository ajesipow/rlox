use std::path::PathBuf;

use clap::Parser;
use rlox::error::PublicError;
use rlox::read_source_file;
use rlox::run_prompt;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // The path of a rlox source file to interpret
    #[arg(short, long)]
    file: Option<PathBuf>,
}

fn main() -> Result<(), PublicError> {
    let args = Args::parse();

    if let Some(file_path) = args.file {
        // TODO put this behind struct
        let raw_source = read_source_file(&file_path)?;
    } else {
        // TODO put this behind struct
        run_prompt()?;
    }
    println!("Hello, world!");
    Ok(())
}
