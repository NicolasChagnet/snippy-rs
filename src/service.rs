use crate::{model::Snippet, storage};
use anyhow::{Context, Result};
use arboard::Clipboard;
use console::{style, Term};
use dialoguer::FuzzySelect;

// Add a snippet whose content is pulled from the clipboard
pub fn add_snippet(db: &mut storage::DatabaseModel, name: &str, description: &str) -> Result<()> {
    // Load clipboard!
    let mut clipboard = Clipboard::new().with_context(|| "Error loading clipboard!")?;
    // Get the content of the clipboard! It returns an error if it is not text or is empty, among other access issues.
    let content = clipboard
        .get_text()
        .with_context(|| "Error reading clipboard (it might be empty)!")?;
    let content_trimmed = content.trim();
    if name.is_empty() || content_trimmed.is_empty() {
        eprintln!("Trying to set an empty key or value!");
        std::process::exit(1);
    }
    let description_to_add = if description.is_empty() {
        "No description provided"
    } else {
        description
    };
    let snippet = storage::set_snippet(db, name, description_to_add, &content_trimmed)?;
    Ok(println!(
        "Snippet set: ({}) - {}",
        snippet.get_name(),
        snippet.get_description()
    ))
}

// Remove a snippet by name
pub fn remove_snippet(db: &mut storage::DatabaseModel, name: &str) -> Result<()> {
    storage::remove_snippet(db, name)?;
    println!("Removed snippet: ({})", name);
    Ok(())
}

// Formats the snippets to be displayed
fn get_format_snippets(snippets: &[Snippet]) -> Vec<String> {
    snippets
        .iter()
        .map(|snippet| {
            format!(
                "{} | {}",
                style(snippet.get_name()).blue(),
                snippet.get_description()
            )
        })
        .collect()
}

// Let the user choose a snippet, set to clipboard
pub fn list_snippets(db: &mut storage::DatabaseModel) -> Result<()> {
    let snippets = storage::get_all_snippets(db)?; // Get all snippets
    let snippets_format: Vec<String> = get_format_snippets(&snippets);
    // Clear the terminal
    Term::stdout().clear_screen()?;
    // Prompt the user to choose one
    let choice: Option<usize> = FuzzySelect::new()
        .with_prompt("Choose a snippet to copy (Esc to cancel):")
        .report(false)
        .default(0)
        .items(&snippets_format)
        .interact_opt()
        .with_context(|| "Error when parsing selection!")?;
    if let Some(choice_int) = choice {
        let chosen_snippet = &snippets[choice_int];
        // Load clipboard!
        let mut clipboard = Clipboard::new().with_context(|| "Error loading clipboard!")?;
        // Set to clipboard
        clipboard
            .set_text(chosen_snippet.get_content())
            .with_context(|| "Error setting content to clipboard!")?;
        println!("Snippet ({}) set to clipboard!", chosen_snippet.get_name())
    }
    Ok(())
}
