use crate::cmd;
use std::collections::HashMap;

pub struct CommandRegistry {
    commands: HashMap<String, Box<dyn cmd::Command>>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        let mut registry = CommandRegistry {
            commands: HashMap::new(),
        };
        // registers commands
        registry.register_command("cd", Box::new(cmd::CdCommand));
        registry.register_command("ls", Box::new(cmd::LsCommand));
        registry.register_command("help", Box::new(cmd::HelpCommand));
        registry.register_command("quit", Box::new(cmd::QuitCommand));

        registry
    }

    fn register_command(&mut self, name: &str, command: Box<dyn cmd::Command>) {
        self.commands.insert(name.to_string(), command);
    }

    pub fn get_command(&self, name: &str) -> Option<&Box<dyn cmd::Command>> {
        self.commands.get(name)
    }

    pub fn list_commands(&self) -> Vec<(&str, &str)> {
        self.commands
            .iter()
            .map(|(name, command)| (name.as_str(), command.description()))
            .collect()
    }
}
