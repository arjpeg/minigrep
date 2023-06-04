use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    author = "arjpeg",
    version = "0.1.0",
    about = "A simple CLI tool to recursively search for a pattern in a file or directory."
)]
pub struct Cli {
    /// The pattern string to search for
    pub pattern: String,

    /// The path to the file or directory to search in.
    /// Recursively searches subdirectories.
    #[clap(default_value = ".")]
    pub path: std::path::PathBuf,
}
