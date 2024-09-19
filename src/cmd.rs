use crate::shell::Shell;
use std::fs;
use std::path::PathBuf;

// trait implemented by commands to run
pub trait Command {
    fn run(&self, shell: &mut Shell, args: &[String]) -> Result<(), &'static str>;
}

// implementation of cd command
pub struct CdCommand;

impl Command for CdCommand {
    fn run(&self, shell: &mut Shell, args: &[String]) -> Result<(), &'static str> {
        // Check if arguments are provided
        if args.is_empty() {
            return Err("No directory specified");
        }

        // Create the new path based on the provided argument
        let new_path = if PathBuf::from(&args[0]).is_relative() {
            let mut relative_path = shell.curr_dir.clone();
            relative_path.push(&args[0]);
            relative_path
        } else {
            PathBuf::from(&args[0])
        };

        // Canonicalize the path to resolve `..`, `.`, and other path components
        match fs::canonicalize(&new_path) {
            Ok(canonical_path) => {
                // Check if the path exists and is a directory
                if canonical_path.is_dir() {
                    shell.curr_dir = canonical_path;
                    Ok(())
                } else {
                    Err("The specified path is not a directory")
                }
            }
            Err(_) => Err("Invalid path or directory does not exist"),
        }
    }
}

// implementation of ls command
pub struct LsCommand;
impl Command for LsCommand {
    fn run(&self, shell: &mut Shell, _args: &[String]) -> Result<(), &'static str> {
        println!("Listing directory contents...");

        let paths = fs::read_dir(&shell.curr_dir).unwrap();

        for path in paths {
            println!("Name: {}", path.unwrap().path().display())
        }

        Ok(())
    }
}
