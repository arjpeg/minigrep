use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    /// The pattern string to search for
    pub pattern: String,

    /// The path to the file or directory to search in.
    /// Recursively searches subdirectories.
    #[clap(default_value = ".")]
    pub path: std::path::PathBuf,
}
