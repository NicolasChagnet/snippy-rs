use humphrey_json::prelude::*;
use std::fmt;
// use uuid::Uuid;

// Struct defining thecontent of a snippet
#[derive(FromJson, IntoJson, Debug)]
pub struct Snippet {
    name: String,
    description: String,
    content: String,
}

// Implement the display trait for this, ignoring the content
impl fmt::Display for Snippet {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "({}) - {}", self.name, self.description)
    }
}

// Implement useful methods to get/set the data from a snippet, as well as a constructor
impl Snippet {
    pub fn new(name: &str, description: &str, content: &str) -> Snippet {
        Snippet {
            // id: Uuid::now_v7().to_string(),
            name: name.to_string(),
            description: description.to_string(),
            content: content.to_string(),
        }
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string()
    }
    pub fn get_description(&self) -> &str {
        &self.description
    }
    pub fn set_description(&mut self, description: &str) {
        self.description = description.to_string()
    }
    pub fn get_content(&self) -> &str {
        &self.content
    }
    pub fn set_content(&mut self, content: &str) {
        self.content = content.to_string()
    }
}
