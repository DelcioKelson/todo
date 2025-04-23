use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name ="todo")]
#[command(about = "A simple CLI todo list manager", long_about = None)]
#[command(version, author)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new task
    Add {
        /// The name of the task
        #[arg(short, long)]
        name: String,
        /// The description of the task
        #[arg(short, long)]
        description: String,
    },
    /// List all tasks
    List,
    /// Remove a task
    Remove {
        /// The ID of the task to remove
        id: usize,
    },
    /// Edit a task
    Edit {
        /// The ID of the task to edit
        #[arg(short, long)]
        id: usize,
        /// The new name of the task
        name: Option<String>,
        /// The new description of the task
        description: Option<String>,
        /// The new status of the task
        #[arg(short, long)]
        status: Option<String>
    },
}
