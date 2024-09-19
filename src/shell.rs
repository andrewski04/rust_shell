use crate::cmd_registry::CommandRegistry;
use crate::config::Config;
use std::env::current_dir;
use std::{
    io::{self, Write},
    path::PathBuf,
};

pub struct Shell {
    config: Config,
    pub curr_dir: PathBuf,
    pub close_shell: bool,
}

impl Shell {
    // builds the shell object, containing configuration options
    pub fn build(config: Config) -> Shell {
        let curr_dir = current_dir().unwrap();
        Shell {
            config,
            curr_dir,
            close_shell: false,
        }
    }

    // Starts the shell execution. this will return an `Ok(())` when gracefully exited,
    // otherwise it returns an error with proper message
    pub fn spawn(&mut self) -> Result<(), &'static str> {
        let registry = CommandRegistry::new();
        while !self.close_shell {
            // creates commands input prompt and replaces placeholder with the current directory
            let curr_dir_str = self.curr_dir.to_str().unwrap();
            let prompt = self
                .config
                .style
                .replace("{curr_dir}", curr_dir_str.replace("\\\\?\\", "").as_str()); //fix windows long dir formatting
            let user_input = input_handler(prompt).unwrap();

            if user_input.is_empty() {
                continue;
            }

            if let Some(command) = registry.get_command(&user_input[0]) {
                command
                    .run(self, &user_input[1..])
                    .unwrap_or_else(|e| eprintln!("{}", e));
            } else {
                println!("Command not found!");
            }
        }

        Ok(())
    }
}

// this takes a string that is display to the user, and returns the users input
fn input_handler(prompt: String) -> io::Result<Vec<String>> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let split_buffer: Vec<String> = buffer.split_whitespace().map(|s| s.to_string()).collect();
    Ok(split_buffer)
}
