use clap::{Parser, Subcommand};
use snippy_rs::{service, storage};

/// Handy snippets manager which imports and exports snippets directly to your clipboard!
#[derive(Debug, Parser)]
#[clap(name = "snippy-rs", bin_name = "snippy", version)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Add a snippet
    #[command(arg_required_else_help = true)]
    Set {
        /// The name of the snippet
        name: String,
        /// The description of the snippet
        description: Option<String>,
    },
    /// Edit a snippet
    #[command(arg_required_else_help = false)]
    Edit,
    /// Remove a snippet
    #[command(arg_required_else_help = false)]
    Del,
}

fn main() {
    let args = Cli::parse(); // Parse CLI args
                             // Load the json content
    let mut db = storage::connect_db().expect("Issue loading the json database file!");

    match args.command {
        Some(Commands::Set { name, description }) => {
            let description_content = description.unwrap_or("".to_string());
            service::add_snippet(&mut db, &name, &description_content)
        }
        .expect("Error encountered when adding snippet!"),
        Some(Commands::Edit) => {
            service::edit_snippet(&mut db).expect("Error encountered when editing snippet!")
        }
        Some(Commands::Del) => {
            service::delete_snippet(&mut db).expect("Error encountered when removing snippet!")
        }

        None => service::choose_snippet(&mut db).expect("Error encountered when listing snippets!"),
    }
}
