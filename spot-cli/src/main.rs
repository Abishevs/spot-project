extern crate spot_lib;

use clap::{Args, Parser, Subcommand};
use std::os::unix::net::UnixStream;
use std::io::{Write, BufRead, BufReader};
use serde_json;

use spot_lib::commands::{MainCommand, PomodoroCommand, Response};

#[derive(Parser)]
#[command(name = "spot",
          version = "1.0",
          about = "Software/Studying Pomodoro Organiser & Tracker",
          long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: SubCommands,
    #[clap( short, long, default_value_t = false, help = "Verbose mode- global")]
    verbose: bool
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
    Start {
        #[clap(short, long, default_value_t = 25, help = "Duration of the Pomodoro in minutes.")]
        duration: u64,
        #[clap(short = 'b', long = "break", default_value_t = 5, help = "Break time in minutes after each Pomodoro.")]
        break_time: u64
    },
    Stop,
    Status,
}

impl From<PomodoroCommands> for PomodoroCommand {
    fn from(cmd: PomodoroCommands) -> Self {
        match cmd {
            PomodoroCommands::Start {duration, break_time} => PomodoroCommand::Start { duration, break_time },
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
            let command = PomodoroCommand::from(pomodoro.command.clone());
            let main_command = MainCommand::Pomodoro(command);
            let command_json = serde_json::to_string(&main_command)?;
            stream.write_all(command_json.as_bytes())?;
            stream.write_all(b"\n")?;

            if cli.verbose {
                let mut reader = BufReader::new(stream);
                let mut response = String::new();
                reader.read_line(&mut response)?;
                print!("{}", response);
            }
        }
    }

    Ok(())
}
