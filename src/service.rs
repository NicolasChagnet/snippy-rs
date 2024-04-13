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
        "No description"
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

// Copies content of a snippet to clipboard
pub fn copy_snippet_hook(_db: &mut storage::DatabaseModel, snippet: &Snippet) -> Result<()> {
    let mut clipboard = Clipboard::new().with_context(|| "Error loading clipboard!")?;
    // Set to clipboard
    clipboard
        .set_text(snippet.get_content())
        .with_context(|| "Error setting content to clipboard!")?;
    println!("Snippet ({}) set to clipboard!", snippet.get_name());
    Ok(())
}

// Edits snippets with current content of clipboard
pub fn edit_snippet_hook(db: &mut storage::DatabaseModel, snippet: &Snippet) -> Result<()> {
    add_snippet(db, snippet.get_name(), snippet.get_description())?;
    Ok(())
}

pub fn delete_snippet_hook(db: &mut storage::DatabaseModel, snippet: &Snippet) -> Result<()> {
    remove_snippet(db, snippet.get_name())?;
    Ok(())
}

// Let the user choose a snippet, set to clipboard
pub fn list_snippets(
    db: &mut storage::DatabaseModel,
    action: fn(&mut storage::DatabaseModel, &Snippet) -> Result<()>,
) -> Result<()> {
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
        action(db, &snippets[choice_int])?;
    }
    Ok(())
}

pub fn choose_snippet(db: &mut storage::DatabaseModel) -> Result<()> {
    list_snippets(db, copy_snippet_hook)?;
    Ok(())
}

pub fn edit_snippet(db: &mut storage::DatabaseModel) -> Result<()> {
    list_snippets(db, edit_snippet_hook)?;
    Ok(())
}

pub fn delete_snippet(db: &mut storage::DatabaseModel) -> Result<()> {
    list_snippets(db, delete_snippet_hook)?;
    Ok(())
}
