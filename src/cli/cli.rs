use clap::Parser;
use lazy_static::lazy_static;

#[derive(Parser)]
struct Arguments {
    #[arg(short, long)]
    file: String,
}

pub struct Configuration {
    pub file: String,
}

lazy_static! {
    pub static ref CONFIG: Configuration = parse_args();
}

fn parse_args() -> Configuration {
    let args = Arguments::parse();
    Configuration {
        file: args.file
    }
}