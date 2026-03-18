use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Todo App", version = "1.0", author = "Medi Poerwito", about = "A simple CLI todo application")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new todo item
    Add { task: String },
    List,
    Complete { id: usize },
    Remove { id: usize }
}


fn main() {
    // let args: Vec<String> = env::args().collect();
    // println!("Arguments: {:?}", args);
    let arg = Args::parse();
    match arg.command {
        Commands::Add { task } => {
            println!("Adding task: {}", task);
            // Here you would add the task to your storage (e.g., a file or database)
        }
        Commands::List => {
            println!("Listing all tasks...");
            // Here you would read and display all tasks from your storage
        }
        Commands::Complete { id } => {
            println!("Completing task with ID: {}", id);
            // Here you would mark the task as completed in your storage
        }
        Commands::Remove { id } => {
            println!("Removing task with ID: {}", id);
            // Here you would remove the task from your storage
        }
    }
}
