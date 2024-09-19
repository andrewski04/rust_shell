use crate::cmd_registry::CommandRegistry;
use crate::shell::Shell;
use std::fs;
use std::path::PathBuf;
// trait implemented by commands to run
pub trait Command {
    fn run(&self, shell: &mut Shell, args: &[String]) -> Result<(), &'static str>;
    fn description(&self) -> &str;
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

    fn description(&self) -> &str {
        "Changes the current directory."
    }
}

// implementation of ls command
pub struct LsCommand;
impl Command for LsCommand {
    fn run(&self, shell: &mut Shell, _args: &[String]) -> Result<(), &'static str> {
        // check for "all" or "-a" arg
        let show_all = _args.iter().any(|arg| arg == "all" || arg == "-a");

        // reads current dir
        let paths = fs::read_dir(&shell.curr_dir).map_err(|_| "Failed to read the directory\n")?;
        println!();

        for path in paths {
            let entry = path.map_err(|_| "Failed to read a directory entry")?;
            let path = entry.path();

            // skip hidden files if no `all` arg
            if !show_all {
                if let Some(file_name) = path.file_name() {
                    if file_name.to_string_lossy().starts_with('.') {
                        continue;
                    }

                    // print only file name
                    println!("{}", file_name.to_string_lossy());
                }
            } else {
                // print full path for `all` arg
                println!("{}", path.display());
            }
        }

        println!();
        Ok(())
    }

    fn description(&self) -> &str {
        "ls [-a | all] - Lists the contents of the current directory."
    }
}

pub struct HelpCommand;

impl Command for HelpCommand {
    fn run(&self, _shell: &mut Shell, _args: &[String]) -> Result<(), &'static str> {
        println!("\nAvailable commands:\n");

        for (name, description) in CommandRegistry::new().list_commands() {
            println!("{} - {}", name, description);
        }

        println!();
        Ok(())
    }

    fn description(&self) -> &str {
        "Displays this help message."
    }
}

pub struct QuitCommand;
impl Command for QuitCommand {
    fn run(&self, mut shell: &mut Shell, _args: &[String]) -> Result<(), &'static str> {
        shell.close_shell = true;
        Ok(())
    }
    fn description(&self) -> &str {
        "Exits the shell."
    }
}
