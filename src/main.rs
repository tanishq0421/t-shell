use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() {
    let vars: HashMap<String, String> = std::env::vars().into_iter().collect(); // Add type annotation
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "exit" {
            break;
        }
        run_process(&vars, input, &vars["PATH"]).unwrap(); // Pass 3rd argument as the PATH
    }
}

fn run_process(vars: &HashMap<String, String>, command: &str, path: &str) -> Result<(), ()> {
    let command_parts: Vec<&str> = command.split(" ").collect();
    let bin = find_binary(command_parts[0], path); // Use path variable
    println!("{:?}", bin);
    Ok(())
}

fn find_binary(command: &str, path: &str) -> Result<PathBuf, std::io::Error> {
    fn search(command: &str, path: &Path) -> Result<(), std::io::Error> {
        for entry in fs::read_dir(path)? {
            if let Ok(entry) = entry {
                let met = entry.metadata()?; // Fetch metadata here
                if met.is_file() || met.is_symlink() {
                    if let Some(name) = entry.path().file_name() {
                        if name == command {
                            if met.is_symlink() {
                                panic!("Running symlink not supported");
                            }
                            return Ok(());
                        }
                    }
                }
            }
        }
        Err(std::io::ErrorKind::NotFound.into()) // Corrected the path separator
    }

    // Check current directory first
    if let Ok(mut dir) = std::env::current_dir() {
        if let Ok(()) = search(command, &dir) {
            dir.push(command);
            return Ok(dir);
        }
    }

    // Search in PATH directories
    for entry in path.split(":") {
        let mut path = PathBuf::from(entry);
        if let Ok(()) = search(command, &path) {
            path.push(command);
            return Ok(path);
        }
    }

    Err(std::io::ErrorKind::NotFound.into())
}
