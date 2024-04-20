use clap::{Args, Parser, Subcommand};
use spot_lib::commands::PomodoroCommand;

#[derive(Parser)]
#[command(name = "spot",
          version = "1.0",
          about = "Software/Studying Pomodoro Organiser & Tracker",
          long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: SubCommands,
    #[clap( short, long, default_value_t = false, help = "Verbose mode- global")]
    pub verbose: bool
}

#[derive(Subcommand)]
pub enum SubCommands {
    Session(Session),
    Pomodoro(Pomodoro),
    Config(Config),
}

#[derive(Args)]
pub struct Config {
    #[arg(help = "Set the project name.")]
    pub project_name: Option<String>,

}


#[derive(Args)]
pub struct Session {
    #[command(subcommand)]
    pub command: SessionCommands,
}

#[derive(Subcommand, Clone)]
pub enum SessionCommands {
    New {
        tags: Vec<String>,
        name: String,
    },
    Start,
    Stop,
}

#[derive(Args)]
pub struct Pomodoro {
    #[command(subcommand)]
    pub command: PomodoroCommands,
}

#[derive(Subcommand, Clone)]
pub enum PomodoroCommands {
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
