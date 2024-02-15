use clap::{Parser, Subcommand};
use snippy_rs::{service, storage};

/// Handy snippets manager which imports and exports snippets directly to your clipboard!
#[derive(Debug, Parser)]
#[clap(name = "snippy", bin_name = "snippy", version)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Add a snippet
    #[command(arg_required_else_help = true)]
    Set {
        /// The name of the snippet
        name: String,
        description: String,
    },
    /// List snippets
    List,
    /// Remove a snippet
    #[command(arg_required_else_help = true)]
    Remove {
        /// The name of the snippet
        name: String,
    },
}

fn main() {
    let args = Cli::parse(); // Parse CLI args
                             // Load the json content
    let mut db = storage::connect_db().expect("Issue loading the json database file!");

    match args.command {
        Commands::Set { name, description } => service::add_snippet(&mut db, &name, &description)
            .expect("Error encountered when adding snippet!"),
        Commands::List => {
            service::list_snippets(&mut db).expect("Error encountered when listing snippets!")
        }
        Commands::Remove { name } => service::remove_snippet(&mut db, &name)
            .expect("Error encountered when removing snippet!"),
    }
}
