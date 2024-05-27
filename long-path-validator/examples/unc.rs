use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::fs::DirBuilder;
use std::process::Command;

fn to_unc_path(path: &Path) -> String {
    let path_str = path.to_str().unwrap();
    if path_str.starts_with(r"\\?\") {
        path_str.to_string()
    } else {
        let canonical_path = path.canonicalize().unwrap();
        format!(r"\\?\{}", canonical_path.display())
    }
}

fn create_and_test_nested_directories(base_path: &PathBuf, depth: usize) -> io::Result<()> {
    let mut current_path = fs::canonicalize(base_path)?;

    for i in 1..=depth {
        let new_dir = format!("level-{:04}", i);
        current_path.push(new_dir);

        // Always convert to UNC path
        let current_path_str = to_unc_path(&current_path);
        println!("Current path: {:?}", current_path_str);

        // Check if the directory exists before creating it
        if !Path::new(&current_path_str).exists() {
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
        let exe_source = fs::canonicalize("hello.exe")?;
        let exe_destination = Path::new(&current_path_str).join("hello.exe");

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

        // Execute hello.exe using UNC path
        let exe_destination_str = to_unc_path(&exe_destination);

        match Command::new(&exe_destination_str).output() {
            Ok(output) => {
                println!("Execution successful at level {}", i);
                println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
                println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
            }
            Err(e) => {
                eprintln!("Failed to execute hello.exe at level {}: {:?}, path: {:?}", i, e, exe_destination_str);
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
