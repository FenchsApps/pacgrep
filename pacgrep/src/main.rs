mod cli;
mod pacman;
mod package;

use crate::cli::{Cli, Commands};
use crate::pacman::{get_installed_packages, get_owner_of_file};
use crate::package::Package;
use clap::Parser;
use regex::Regex;
use anyhow::{anyhow, Result};
use std::collections::HashMap;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Find { name, description, size, depends_on, format } => {
            let packages = get_installed_packages()?;

            let mut filtered_packages: Vec<&Package> = packages.iter().collect();

            if let Some(name_regex) = name {
                let re = Regex::new(name_regex)?;
                filtered_packages.retain(|p| re.is_match(&p.name));
            }

            if let Some(desc_regex) = description {
                let re = Regex::new(desc_regex)?;
                filtered_packages.retain(|p| re.is_match(&p.description));
            }

            if let Some(dep_regex) = depends_on {
                let re = Regex::new(dep_regex)?;
                filtered_packages.retain(|p| p.depends.iter().any(|d| re.is_match(d)));
            }

            // A simple parser for size, e.g., ">10M"
            if let Some(size_str) = size {
                let op = size_str.chars().next().ok_or_else(|| anyhow!("Invalid size format"))?;
                let value_str = &size_str[1..];
                let value = value_str.trim_end_matches(['M', 'G']).parse::<u64>()?;
                let factor = if size_str.ends_with('M') { 1_000_000 } else if size_str.ends_with('G') { 1_000_000_000 } else { 1 };
                let size_bytes = value * factor;

                filtered_packages.retain(|p| match op {
                    '>' => p.size > size_bytes,
                    '<' => p.size < size_bytes,
                    '=' => p.size == size_bytes,
                    _ => false,
                });
            }

            for pkg in filtered_packages {
                let mut output = format.clone();
                output = output.replace("{name}", &pkg.name);
                output = output.replace("{version}", &pkg.version);
                output = output.replace("{size}", &format!("{} bytes", pkg.size));
                output = output.replace("{description}", &pkg.description);
                println!("{}", output);
            }
        }
        Commands::File { path } => {
            match get_owner_of_file(path) {
                Ok(owner) => println!("The file '{}' is owned by package: {}", path, owner),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }

    Ok(())
}
