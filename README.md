# snippy-rs
A simple cross-platform CLI tool to manage user-defined snippets written in Rust. Snippets are loaded and unloaded through the clipboard. The snippets are stored in JSON format inside the application folder located inside the default user data folder for the platform. These are obtained through the crate [directories](https://crates.io/crates/directories):

- `$HOME/.local/share` for Linux,
- `$HOME/Library/Application Support` for MacOS,
- `%UserProfile%\AppData\Roaming` for Windows.

