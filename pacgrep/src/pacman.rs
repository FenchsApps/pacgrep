use crate::package::Package;
use anyhow::{Context, Result};
use std::process::{Command, Stdio};
use std::io::{self, BufRead, BufReader};

/// Fetches all installed packages and their details by parsing `pacman -Qi` output.
pub fn get_installed_packages() -> Result<Vec<Package>> {
    // We use `pacman -Q` to get a list of all installed packages, then `pacman -Qi` for details.
    let output = Command::new("pacman")
        .args(["-Q"])
        .output()
        .context("Failed to run 'pacman -Q'")?;

    let stdout = String::from_utf8(output.stdout)?;
    let packages: Vec<&str> = stdout.lines().collect();
    let mut details = Vec::new();

    // Loop through each package to get its full info.
    for pkg in packages {
        let pkg_name = pkg.split_whitespace().next().unwrap_or_default();
        let detail_output = Command::new("pacman")
            .args(["-Qi", pkg_name])
            .output()
            .context(format!("Failed to get details for package: {}", pkg_name))?;

        let detail_str = String::from_utf8(detail_output.stdout)?;

        // Simple parsing of `pacman -Qi` output.
        // A more robust solution might use a dedicated parser or a different pacman command.
        let mut package = Package {
            name: pkg_name.to_string(),
            version: String::new(),
            description: String::new(),
            size: 0,
            depends: Vec::new(),
        };

        for line in detail_str.lines() {
            if line.starts_with("Version") {
                package.version = line.split(":").nth(1).unwrap_or_default().trim().to_string();
            } else if line.starts_with("Description") {
                package.description = line.split(":").nth(1).unwrap_or_default().trim().to_string();
            } else if line.starts_with("Installed Size") {
                let size_str = line.split(":").nth(1).unwrap_or_default().trim().split_whitespace().next().unwrap_or("0");
                package.size = size_str.parse().unwrap_or(0);
            } else if line.starts_with("Depends On") {
                let depends_str = line.split(":").nth(1).unwrap_or_default().trim();
                package.depends = depends_str.split_whitespace().map(|s| s.to_string()).collect();
            }
        }
        details.push(package);
    }

    Ok(details)
}

/// Finds the package that owns a specific file using `pacman -Qo`.
pub fn get_owner_of_file(file_path: &str) -> Result<String> {
    let output = Command::new("pacman")
        .args(["-Qo", file_path])
        .output()
        .context("Failed to run 'pacman -Qo'")?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("File does not belong to any package."));
    }

    let stdout = String::from_utf8(output.stdout)?;
    // Example output: `/usr/bin/python is owned by python 3.10.4-1`
    let parts: Vec<&str> = stdout.trim().split_whitespace().collect();
    let package_name = parts[3].to_string();

    Ok(package_name)
}
