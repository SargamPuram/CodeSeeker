use std::fs;
use std::io::{self, BufRead};

fn search_file(file_path: &str, pattern: &str) -> io::Result<()> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for (line_number, line) in reader.lines().enumerate() {
        let line = line?;
        if line.contains(pattern) {
            println!("{}:{} - {}", file_path, line_number + 1, line);
        }
    }

    Ok(())
}

fn search_directory(directory_path: &str, pattern: &str) -> io::Result<()> {
    for entry in fs::read_dir(directory_path)? {
        let entry = entry?;
        let file_path = entry.path();

        if file_path.is_file() {
            if let Some(file_name) = file_path.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    search_file(&file_name_str, pattern)?;
                }
            }
        }
    }

    Ok(())
}

fn main() {

    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <directory_path> <pattern>", args[0]);
        std::process::exit(1);
    }

    let directory_path = &args[1];
    let pattern = &args[2];

    if let Err(err) = search_directory(directory_path, pattern) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
