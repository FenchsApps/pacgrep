
# pacgrep

![Rust](https://img.shields.io/badge/rust-1.78.0-orange?style=for-the-badge&logo=rust)
![Arch Linux](https://img.shields.io/badge/Arch_Linux-1793D1?style=for-the-badge&logo=arch-linux&logoColor=white)

---

**`pacgrep`** is a powerful and fast command-line utility for Arch Linux, written in Rust. It extends and enhances the standard `pacman` capabilities for searching and filtering packages, making package management more intuitive and efficient.

## Why use `pacgrep`?

Standard `pacman` commands like `pacman -Ss` and `pacman -Ql` can often produce too much information. `pacgrep` solves this by providing a single, intuitive interface for:

* **Advanced searching** by combining multiple criteria at once.
* **Flexible filtering** of search results.
* **Customizable output** to display only the data you need.
* **Quickly identifying** the owner of a file.

## Installation

To install `pacgrep`, make sure you have **Rust** and **Cargo** installed. Then, clone the repository and build the project:

```bash
# Clone the repository
git clone [https://github.com/YOUR_GITHUB_USERNAME/pacgrep.git](https://github.com/YOUR_GITHUB_USERNAME/pacgrep.git)

# Navigate to the project directory
cd pacgrep

# Build and install the utility
cargo install --path .
````

After installation, the utility will be available in your terminal as `pacgrep`.

## Usage

### 1\. Finding Packages

Use the `find` subcommand to search for packages.

```bash
# Find a package by name using a regular expression
pacgrep find --name "firefox"

# Find a package with a description containing "web server"
pacgrep find --description "web server"

# Find packages larger than 100 MB
pacgrep find --size ">100M"

# Find packages that depend on "curl"
pacgrep find --depends-on "curl"
```

You can combine multiple filters for more precise searches:

```bash
# Find packages with a name containing "python" that also depend on "openssl"
pacgrep find --name "python" --depends-on "openssl"
```

### 2\. Customizing Output

Use the `--format` option to control how results are displayed. Available fields are: `{name}`, `{version}`, `{size}`, and `{description}`.

```bash
# Display only the name and version
pacgrep find --name "kernel" --format "{name} - {version}"

# Display the name and size
pacgrep find --name "bash" --format "{name} ({size})"
```

### 3\. Finding a File's Owner

Use the `file` subcommand to determine which package owns a specific file.

```bash
pacgrep file /usr/bin/python
```

### 4\. Help

For a full list of commands and options, use the `--help` flag:

```bash
pacgrep --help
pacgrep find --help
```
