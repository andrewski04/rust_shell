mod cmd;
mod cmd_registry;
mod config;
mod shell;

use config::Config;
use shell::Shell;

fn main() {
    // Attempts to create Config from toml file at path, throws error if failed
    let config: Config = match Config::build("./config.toml") {
        Ok(c) => c,
        Err(e) => {
            panic!("Failed with error: {}", e);
        }
    };

    // build new shell obj from config
    let mut shell = Shell::build(config);

    // spawns shell process returns shell result for a graceful shutdown or unhandled error
    match shell.spawn() {
        Ok(_) => {
            println!("Shell session ended gracefully.");
            std::process::exit(0);
        }
        Err(e) => {
            panic!("Shell ended with error: {}", e);
        }
    }
}
