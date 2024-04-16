extern crate spot_lib;

use clap::{Args, Parser, Subcommand};
use std::os::unix::net::UnixStream;
use std::io::{Write, BufRead, BufReader};
use serde_json;

use spot_lib::commands::{PomodoroCommand, Response};

#[derive(Parser)]
#[command(name = "spot", version = "1.0", about = "Software/Studying Pomodoro Organiser & Tracker", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: SubCommands,
}

#[derive(Subcommand)]
enum SubCommands {
    Config(Config),
    Pomodoro(Pomodoro),
}

#[derive(Args)]
struct Config {
    #[arg(help = "Set the project name.")]
    project_name: Option<String>,

}

#[derive(Args)]
struct Pomodoro {
    #[command(subcommand)]
    command: PomodoroCommands,
}

#[derive(Subcommand, Clone)]
enum PomodoroCommands {
    Start,
    Stop,
    Status,
}

impl From<PomodoroCommands> for PomodoroCommand {
    fn from(cmd: PomodoroCommands) -> Self {
        match cmd {
            PomodoroCommands::Start => PomodoroCommand::Start,
            PomodoroCommands::Stop => PomodoroCommand::Stop,
            PomodoroCommands::Status => PomodoroCommand::Status,
        }
    }
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let mut stream = UnixStream::connect("/tmp/spot-deamon.socket")?;
    match &cli.command {
        SubCommands::Config(config) => {
            if let Some(name) = &config.project_name {
                println!("Configuring project name to: {}", name);
            }
        }
        SubCommands::Pomodoro(pomodoro) => {
            // match pomodoro.command {
            //     PomodoroCommands::Start => {
            //         stream.write_all(b"start\n")?;
            //     }
            //     PomodoroCommands::Stop => {
            //         stream.write_all(b"stop\n")?;
            //     }
            //     PomodoroCommands::Status => {
            //         stream.write_all(b"status\n")?;
            //     }
            // }
            let command = PomodoroCommand::from(pomodoro.command.clone());
            let command_json = serde_json::to_string(&command)?;
            stream.write_all(command_json.as_bytes())?;
            stream.write_all(b"\n")?;

            let mut reader = BufReader::new(stream);
            let mut response = String::new();
            reader.read_line(&mut response)?;
            println!("{}", response);
        }
    }

    Ok(())
}
