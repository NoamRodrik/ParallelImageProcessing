mod cli;

fn main() {
    println!("file: {}!", &cli::configuration().file);
}