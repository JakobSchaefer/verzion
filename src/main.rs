use clap::{Parser, Subcommand};

/// Verzion is a CLI tool that helps with versioning and changelog generation in CI/CD pipelines.
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Current,
    Next,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Current => println!("0.0.0"),
        Command::Next => println!("0.1.0"),
    }
}
