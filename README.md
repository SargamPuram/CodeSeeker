# CodeSeeker
A command line Code Search tool in Rust
# Code Search Tool in Rust

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

This Rust program is a simple code search tool designed to find a specified pattern within the content of files in a given directory. It provides a quick way to identify and display lines in those files that contain the specified search pattern.

## Key Features

- **Search a Directory:**
  - The tool takes a directory path and a search pattern as command-line arguments.

- **Iterate Over Files:**
  - It iterates over all the files in the specified directory.

- **Search Within Files:**
  - For each file, it opens the file and searches for lines containing the specified pattern.

- **Display Results:**
  - If a line in a file matches the search pattern, it prints the file path, line number, and the content of the matching line.

## How to Use

### Build the Code

Use the following command to build the Rust executable:

```bash
cargo build

Run the Code Search Tool
Execute the tool by providing the directory path and the search pattern as command-line arguments:

'''bash
./target/debug/CodeSeeker /path/to/your/codebase "your_search_pattern"

###Customization
Path Format:

Ensure that the provided directory path is accurate and points to the directory you want to search.
Search Pattern:

Adjust the search pattern to match the specific text you are looking for in your codebase.
Output:

The tool will print lines in files that contain the specified pattern, along with file paths and line numbers.
###Expected Output
Upon running the tool, you should see output similar to the following:

plaintext
Copy code
/path/to/your/codebase/file1.txt:42 - Line containing your_search_pattern
/path/to/your/codebase/file2.rs:7 - Another line with your_search_pattern
License
This code search tool is open-source and available under the MIT License.

Feel free to contribute