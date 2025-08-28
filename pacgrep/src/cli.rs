use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about = "A powerful tool for searching and filtering Arch Linux packages.", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Find packages based on various criteria
    Find {
        /// Regex to search for a package name
        #[arg(short, long)]
        name: Option<String>,

        /// Regex to search within a package description
        #[arg(short, long)]
        description: Option<String>,

        /// Filter by package size (e.g., '>10M', '<5G')
        #[arg(short, long)]
        size: Option<String>,

        /// Regex to search for dependencies
        #[arg(short = 'e', long)]
        depends_on: Option<String>,

        /// Optional output format. Available fields: {name}, {version}, {size}, {description}
        #[arg(short, long, default_value = "{name} ({version}) - {description}")]
        format: String,
    },
    /// Find which package owns a specific file
    File {
        /// The path to the file
        path: String,
    },
}
