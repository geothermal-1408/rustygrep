use std::env;
use std::fs;

use minigrep::{case_insensitive, case_sensitive};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: cargo run <query> <file_path> [--case-sensitive]");
        eprintln!("  <query>: The text to search for");
        eprintln!("  <file_path>: Path to the file to search in");
        eprintln!("  --case-sensitive (optional): Enable case-sensitive search");
        std::process::exit(1);
    }

    let config = parse_config(&args);
    
    let contents = fs::read_to_string(&config.file_path)
        .expect("Should be able to read the file");

    let results = match config.search_type {
        SearchType::CaseInsensitive => case_insensitive(&config.query, &contents),
        SearchType::CaseSensitive => case_sensitive(&config.query, &contents),
    };
    
    print_results(&results);
}


struct Config {
    query: String,
    file_path: String,
    search_type: SearchType,
}

enum SearchType {
    CaseSensitive,
    CaseInsensitive,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    let search_type = if args.len() > 3 && args[3] == "--case-sensitive" {
        SearchType::CaseSensitive
    } else {
        SearchType::CaseInsensitive
    };

    Config {
        query,
        file_path,
        search_type,
    }
}

fn print_results(lines: &[&str]) {
    println!("[INFO]: Found {} match(es):", lines.len());
    for (i, line) in lines.iter().enumerate() {
        println!("{}: {}", i + 1, line);
    }
}