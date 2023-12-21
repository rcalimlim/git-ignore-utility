mod cli;

use clap::*;
use cli::Args;

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
