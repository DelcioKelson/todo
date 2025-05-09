mod cli;
mod commands;
mod task;
mod storage;

use cli::{Cli, Commands};
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { name, description } => {
            commands::add::run(name, description);
        }
        Commands::List => {
            commands::list::run();
        }
        Commands::Remove { id } => {
            commands::remove::run(id);
        }
        Commands::Edit { id, name, description , status} => {
            commands::edit::run(id, name, description, status);
        }
    }
}