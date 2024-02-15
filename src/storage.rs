// use anyhow::*;
use crate::model::Snippet;
use anyhow::{bail, Context, Result};
use directories::ProjectDirs;
use jasondb::Database;
use std::path::PathBuf;

pub type DatabaseModel = Database<Snippet>;

// Obtain location of database file
pub fn get_location_database() -> Result<PathBuf> {
    let path_root = match ProjectDirs::from("", "", "snippy-rs") {
        Some(proj_dirs) => proj_dirs.data_dir().to_path_buf(),
        None => bail!("Cannot find configuration folder!"),
    };
    std::fs::create_dir_all(&path_root)?; // Creating the config directory if it does not exist!
    Ok(path_root.join("snippets.json"))
}

// DB connection function
pub fn connect_db() -> Result<DatabaseModel> {
    let filename = get_location_database()?;
    let db: DatabaseModel = Database::new(filename).with_context(|| "Error opening database!")?;
    Ok(db)
}

// Sets data to DB
pub fn set_snippet(
    db: &mut DatabaseModel,
    name: &str,
    description: &str,
    content: &str,
) -> Result<Snippet> {
    let snippet = Snippet::new(name, description, content);
    db.set(name, &snippet)
        .with_context(|| "Error setting the snippet in the .json file!")?;
    Ok(snippet)
}

// Delete data from DB
pub fn remove_snippet(db: &mut DatabaseModel, name: &str) -> Result<()> {
    let deletion = db.delete(name);
    match deletion {
        Ok(_) => Ok(()),
        Err(jasondb::error::JasonError::InvalidKey) => {
            eprintln!("The snippet does not exist!");
            std::process::exit(1)
        }
        _ => bail!("Error removing the snippet in the .json file!"),
    }
}

// Gets all snippets from DB
pub fn get_all_snippets(db: &mut DatabaseModel) -> Result<Vec<Snippet>> {
    let snippets_and_keys = db
        .iter()
        .filter_map(|x| x.ok())
        .collect::<Vec<(String, Snippet)>>();
    // The database query returns a vector tuples, we want to extract the Snippet structure from each tuple
    let (_, vec_snippets): (Vec<_>, Vec<_>) = snippets_and_keys.into_iter().unzip();
    Ok(vec_snippets)
}
