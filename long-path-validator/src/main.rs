use std::fs;
use std::io;
use std::path::PathBuf;
use std::fs::DirBuilder;
use std::process::Command;

fn create_and_test_nested_directories(base_path: &PathBuf, depth: usize) -> io::Result<()> {
    let mut current_path = base_path.clone();

    for i in 1..=depth {
        let new_dir = format!("level-{:04}", i);
        current_path.push(new_dir);

        // Convert to UNC path if the path is too long
        let current_path_str = if current_path.as_os_str().len() > 260 {
            format!(r"\\?\{}", current_path.display())
        } else {
            current_path.display().to_string()
        };

        // Check if the directory exists before creating it
        if !PathBuf::from(&current_path_str).exists() {
            match DirBuilder::new().recursive(false).create(&current_path_str) {
                Ok(_) => println!("Created: {:?}", current_path_str),
                Err(e) => {
                    eprintln!("Failed to create directory at level {}: {:?}, path: {:?}", i, e, current_path_str);
                    return Err(e);
                }
            }
        } else {
            println!("Directory already exists: {:?}", current_path_str);
        }

        // Copy hello.exe to the current directory
        let exe_source = PathBuf::from("hello.exe");
        let exe_destination = PathBuf::from(&current_path_str).join("hello.exe");

        // Check if the source file exists before copying
        if !exe_source.exists() {
            eprintln!("Source file does not exist: {:?}", exe_source);
            return Err(io::Error::new(io::ErrorKind::NotFound, "Source file not found"));
        }

        // Check if the file already exists before copying
        if !exe_destination.exists() {
            match fs::copy(&exe_source, &exe_destination) {
                Ok(_) => println!("Copied hello.exe to {:?}", exe_destination),
                Err(e) => {
                    eprintln!("Failed to copy hello.exe at level {}: {:?}, source: {:?}, destination: {:?}", i, e, exe_source, exe_destination);
                    return Err(e);
                }
            }
        } else {
            println!("hello.exe already exists in {:?}", exe_destination);
        }

        // Execute hello.exe
        match Command::new(&exe_destination).output() {
            Ok(output) => {
                println!("Execution successful at level {}", i);
                println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
                println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
            }
            Err(e) => {
                eprintln!("Failed to execute hello.exe at level {}: {:?}, path: {:?}", i, e, exe_destination);
                return Err(e);
            }
        }
    }

    Ok(())
}

fn main() {
    let base_path = PathBuf::from("nested_directories_test");
    let depth = 500;

    match create_and_test_nested_directories(&base_path, depth) {
        Ok(_) => println!("Successfully created and tested all directories up to depth {}", depth),
        Err(_) => println!("Stopped due to an error.")
    }
}
