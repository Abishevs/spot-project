mod cli;
mod fetcher;
mod handler;
mod picker;

use clap::{CommandFactory, Parser};
use clap_complete;
use cli::{Cli, SubCommands};
use fetcher::DaemonClient;

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    let mut daemon_client = DaemonClient::new("/tmp/spot-daemon.socket");
    daemon_client
        .connect()
        .expect("ERROR: Failed to connect to SPOT Daemon, ensure it's active & running");

    let mut handler = handler::CommandHandler::new(&mut daemon_client);

    let res = match &cli.command {
        SubCommands::Config(config) => handler.handle_config(config)?,
        SubCommands::Pomodoro(pomodoro) => handler.handle_pomodoro(&pomodoro.command)?,
        SubCommands::Session(session) => handler.handle_session(&session.command)?,
        SubCommands::Project(project) => handler.handle_project(&project.command)?,
        SubCommands::GenerateCompletions { shell } => {
            let mut app = Cli::command();
            clap_complete::generate(*shell, &mut app, "spot-cli", &mut std::io::stdout());
            return Ok(());
        }
    };
    if cli.verbose {
        println!("{:?}", res);
    }

    Ok(())
}
