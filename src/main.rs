#![forbid(unsafe_code)]
use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(name = "sigma-command-code", about = "SigmaOS Command Code")]
struct Cli { #[command(subcommand)] cmd: Commands }
#[derive(Subcommand)]
enum Commands { List, Status }
fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::List   => println!("sigma-command-code: no entries configured."),
        Commands::Status => println!("sigma-command-code: ready."),
    }
}
