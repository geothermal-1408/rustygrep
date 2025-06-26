# Minigrep

A simple command-line search tool written in Rust that searches for text patterns in files. Similar to the classic `grep` command but with simplified functionality.

## Installation

1. Make sure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/)
2. Clone this repository:

```bash
git clone https://github.com/yourusername/minigrep
cd minigrep
```

## Build the project

```bash
cargo build
```

## Usage

```bash
cargo run <query> <file_path> [--case-sensitive]
```

### Parameters

- `query`: The search string to look for in the file

  - Required
  - Can be any text pattern
  - Case-insensitive by default

- `file_path`: Path to the file to search in

  - Required
  - Must be a valid file path
  - File must be readable text

- `--case-sensitive`: Optional flag for case-sensitive search
  - Optional
  - When present, matches are exact case
  - When absent, search is case-insensitive

### Examples

```bash
# Search case-insensitive (default)
cargo run frog poem.txt

# Search case-sensitive
cargo run Frog poem.txt --case-sensitive

# Search in a specific file path
cargo run pattern ./data/myfile.txt
```
