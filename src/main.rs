use std::path::Path;

use clap::Parser;
use owo_colors::OwoColorize;

pub mod input;

enum Error {
    FileNotFound(String),
}

fn search_file(query: String, file: &Path) -> Result<(), Error> {
    let contents = std::fs::read_to_string(file);

    let contents = match contents {
        Ok(contents) => contents,
        Err(_) => return Err(Error::FileNotFound(file.to_string_lossy().to_string())),
    };

    for (line_number, line) in contents.lines().enumerate() {
        if let Some(span_start) = line.find(&query) {
            let span = span_start..span_start + query.len();

            let line = line.replace(&query, &query.red().bold().to_string());

            println!(
                "{}:{}:  {}
             {}",
                file.to_string_lossy().to_string().blue(),
                (line_number + 1).to_string().yellow(),
                line.trim_start(),
                " ".repeat(span.start) + &"^".repeat(span.len()).red().bold().to_string()
            );
        }
    }

    Ok(())
}

fn search(query: String, path: &Path) -> Result<(), Error> {
    if path.is_dir() {
        for entry in std::fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            search(query.clone(), &path)?;
        }
    } else {
        search_file(query, path)?;
    }

    Ok(())
}

fn main() {
    let args = input::Cli::parse();
    let path = Path::new(&args.path);

    match search(args.pattern, path) {
        Ok(_) => (),
        Err(Error::FileNotFound(file)) => {
            eprintln!("{}: File not found ({})", "Error".red(), file);
            std::process::exit(1);
        }
    }
}
