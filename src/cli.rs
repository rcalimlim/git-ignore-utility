use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    // name of the gitignore file keyword to get
    #[arg(short, long)]
    keyword: String,
}
