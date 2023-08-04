use clap::{Parser, Subcommand};

mod sub_commands;
mod config;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// initiate a new repository
    Init(sub_commands::init::SubCommandArgs),
}


fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init(args) => { 
            sub_commands::init::launch_init(args);
        }
    }
}