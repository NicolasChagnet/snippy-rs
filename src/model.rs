use humphrey_json::prelude::*;
use std::fmt;

// Struct defining thecontent of a snippet
#[derive(FromJson, IntoJson, Debug)]
pub struct Snippet {
    name: String,
    description: String,
    content: String,
}

// Implement the display trait for this structure, ignoring the content field
impl fmt::Display for Snippet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}) - {}", self.name, self.description)
    }
}

// Implement useful methods to get/set the data from a snippet, as well as a constructor
impl Snippet {
    pub fn new(name: &str, description: &str, content: &str) -> Snippet {
        Snippet {
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
