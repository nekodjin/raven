use std::path::PathBuf;
use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version, author, about, long_about = None)]
pub struct Args {
    pub file: PathBuf,
}
