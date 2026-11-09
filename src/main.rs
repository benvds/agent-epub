use clap::{CommandFactory, Parser};

#[derive(Parser)]
#[command(version, about = "Focused EPUB generation for AI agents")]
struct Cli {}

fn main() {
    let _ = Cli::parse();
    Cli::command().print_help().expect("write help");
    println!();
}
