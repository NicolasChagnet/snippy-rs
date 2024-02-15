# snippy-rs
A simple cross-platform CLI tool to manage user-defined snippets written in Rust. Snippets are loaded and unloaded through the clipboard. The snippets are stored in JSON format inside the application folder located inside the default user data folder for the platform. These are obtained through the crate [directories](https://crates.io/crates/directories):

- `$HOME/.local/share` for Linux,
- `$HOME/Library/Application Support` for MacOS,
- `%UserProfile%\AppData\Roaming` for Windows.

# Installation

## Cargo install

You can install this crate with Cargo by doing

```
cargo install snippy-rs
```

## From source

You can build this crate from source using the following commands:
```
> git clone https://github.com/NicolasChagnet/snippy-rs.git
> cd snippy-rs
> cargo build
> cp target/release/snippy /dest/path
```
In the last command, you can copy the binary to any folder contained in your `$PATH`.

**NOTE:** The crate [cli-clipboard](https://crates.io/crates/cli-clipboard) requires the libraries `xord-dev` and `libxcb-composite0-dev` to be built.

# Usage

To set a new snippet, just run
```
snippy set NAME DESCRIPTION
```
The content of the snippet will be pulled from your clipboard and associated with the identifier `NAME`. The description is optional.

The various snippets can then be displayed with
```
snippy
```
which will start a selector with fuzzy search. Pressing `Enter` will copy the snippet's content back to your clipboard while pressing `Esc` exits the menu.

A snippet can be removed by using
```
snippy del NAME
```