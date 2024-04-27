mod fetcher;
mod picker;
mod handler;
mod cli;

use std::process;
use cli::{Cli, SubCommands};
use clap::{CommandFactory, Parser};
use clap_complete;
use fetcher::DaemonClient;

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    let mut daemon_client = DaemonClient::new("/tmp/spot-daemon.socket");
    match daemon_client.connect() {
        Err(_) => {
            eprintln!("ERROR: Failed to connect to SPOT Daemon, ensure it's active");
            process::exit(1);
        }, 
        Ok(daemon_client) => daemon_client,
    };

    let mut handler = handler::CommandHandler::new(&mut daemon_client);

    let res = match &cli.command {
        SubCommands::Config(config) => {
            handler.handle_config(config)?
        }
        SubCommands::Pomodoro(pomodoro) => {
            handler.handle_pomodoro(&pomodoro.command)?

        }
        SubCommands::Session(session) => {
            handler.handle_session(&session.command)?
        }

        SubCommands::Project(project) => {
            handler.handle_project(&project.command)?
        }
        SubCommands::GenerateCompletions { shell } => {
            let mut app = Cli::command();
            clap_complete::generate(*shell, &mut app, "spot-cli", &mut std::io::stdout());
            return Ok(());
        },
    };
    if cli.verbose {
        println!("{:?}", res);
    }

    Ok(())
}
