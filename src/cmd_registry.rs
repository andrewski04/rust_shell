use crate::cmd;
use std::collections::HashMap;

pub struct CommandRegistry {
    commands: HashMap<String, Box<dyn cmd::Command>>,
    aliases: HashMap<String, Box<dyn cmd::Command>>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        let mut registry: CommandRegistry = CommandRegistry {
            commands: HashMap::new(),
            aliases: HashMap::new(),
        };
        // registers commands
        registry.register_command("cd", Box::new(cmd::CdCommand));
        registry.register_command("ls", Box::new(cmd::LsCommand));
        registry.register_command("help", Box::new(cmd::HelpCommand));
        registry.register_command("quit", Box::new(cmd::QuitCommand));
        registry.register_command("clear", Box::new(cmd::ClearCommand));

        registry.register_alias("exit", Box::new(cmd::QuitCommand));
        registry.register_alias("cls", Box::new(cmd::ClearCommand));

        registry
    }

    fn register_command(&mut self, name: &str, command: Box<dyn cmd::Command>) {
        self.commands.insert(name.to_string(), command);
    }

    fn register_alias(&mut self, name: &str, command: Box<dyn cmd::Command>) {
        self.aliases.insert(name.to_string(), command);
    }

    pub fn get_command(&self, name: &str) -> Option<&Box<dyn cmd::Command>> {
        // tries to get command from main registry, otherwise attempts to find it in aliases
        if let Some(command) = self.commands.get(name) {
            return Some(command);
        }
        self.aliases.get(name)
    }

    pub fn list_commands(&self) -> Vec<(&str, cmd::CommandInfo)> {
        let mut command_list: Vec<(&str, cmd::CommandInfo)> = Vec::new();

        // Collect main commands
        for (name, command) in &self.commands {
            command_list.push((name.as_str(), command.description().clone()));
        }

        // Collect aliases
        //for (alias, command) in &self.aliases {
        //    command_list.push((alias.as_str(), command.description().clone()));
        //}

        // Sort alphabetically by command name
        command_list.sort_by(|a, b| a.0.cmp(b.0));

        command_list
    }
}
