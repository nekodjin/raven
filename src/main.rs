use std::fs::File;
use clap::Parser as _;

mod args;
use args::Args;

mod interning;
mod parsing;

fn main() {
    let args = Args::parse();

    let file = File::open(args.file).unwrap();

    let lexer = parsing::lexer::Lexer::new(file);

    for token in lexer {
        println!("{token:#?}");
    }
}
