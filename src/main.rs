use clap::Parser;

#[derive(Parser)]
#[command(
    about, author, version,
)]
struct Cli {}

fn main() {
    // let args = Cli::parse();
    let args = Cli::parse_from(["a", "--help"]);
    println!("Hello, world!");
}
