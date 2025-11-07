use std::fs::File;
use std::io::{self, BufRead, BufReader};

use std::fs::{self};
// use std::fs::{self, DirEntry};
// use std::path::Path;
use std::process::{Command, Stdio};

fn read_file_line_by_line(file_path: &str) -> io::Result<()> {
    // Open the file
    let file = File::open(file_path)?;

    // Create a buffered reader for the file
    let reader = BufReader::new(file);

    // Iterate over each line in the file
    for line in reader.lines() {
        let line = line?; // Unwrap the Result from the iterator
        println!("{}", line); // Print each line
    }

    Ok(())
}

fn list_files_in_directory(dir_path: &str) -> std::io::Result<()> {
    // Read the directory ( returns an iterator )
    let entries = fs::read_dir(dir_path)?;
    // Iterate over each entry in the directory
    for entry in entries {
        let entry = entry?; // Unwrap the Result from the iterator
                            // Print the file name (entry path)
        println!("{}", entry.path().display());
    }

    Ok(())
}

fn list_files_using_ls(dir_path: &str) -> io::Result<()> {
    // Run the `ls` command on the specified directory
    let output = Command::new("ls")
        .arg(dir_path)
        .stdout(Stdio::piped())
        .output()?;

    // Create a BufReader to read the output of the command
    let reader = BufReader::new(&output.stdout[..]);

    // Iterate over each line of the command output (the file names)
    for line in reader.lines() {
        let line = line?; // Unwrap the Result from the iterator
        println!("{}", line); // Print each file name
    }

    Ok(())
}

fn list_processes() -> io::Result<()> {
    // Run the `ps` command to list processes
    let output = Command::new("ps")
        .arg("aux")
        .stdout(Stdio::piped())
        .output()?;

    // Create a BufReader to read the command output
    let reader = BufReader::new(&output.stdout[..]);

    // Iterate over each line in the ps command output
    for (index, line) in reader.lines().enumerate() {
        let line = line?;

        if index > 0 {
            // Skip the header line
            println!("{}", line); // Print each process info
        }
    }

    Ok(())
}
fn main() {
    // Reading lines from file using iterators
    let file_path = "/etc/hosts";
    if let Err(e) = read_file_line_by_line(file_path) {
        eprintln!("Error reading file: {}", e);
    }

    // Iterating over files in a directory
    let dir_path = "/etc/systemd";
    if let Err(e) = list_files_in_directory(dir_path) {
        eprintln!("Error reading Directory!: {}", e);
    }

    //processing output from Systems commands: ( like ls or ps ) and process the output line by
    //line using iterators.
    let dir_path = "/etc/systemd"; // Change to your directory path
    if let Err(e) = list_files_using_ls(dir_path) {
        eprintln!("Error running ls command: {}", e);
    }

    // Iterate over Process ID's ( ps command )
    if let Err(e) = list_processes() {
        eprintln!("Error running ps command: {}", e);
    }
}
