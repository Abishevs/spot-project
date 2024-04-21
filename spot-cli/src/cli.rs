use clap::{Args, Parser, Subcommand};
use spot_lib::{commands::{PomodoroCommand, ProjectCommand}, models::Tag};
use spot_lib::models::Project as ProjectModel;

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
    Project(Project),
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

#[derive(Args)]
pub struct Project {
    #[command(subcommand)]
    pub command: ProjectCommands,
}

#[derive(Subcommand, Clone)]
pub enum ProjectCommands {
    New {
        #[arg(help = "New Project name")]
        name: String,
        #[arg(short, long, help = "Project description")]
        description: Option<String>,
        #[clap(short,
               long,
               help = "Add project tags either with -t tag1 -t tag2 or -t 'tag1,tag2'",
               use_value_delimiter = true)]
        tags: Option<Vec<String>>,
    },
    List,
}

impl From<ProjectCommands> for ProjectCommand {
    fn from(cmd: ProjectCommands) -> Self {
        match cmd {
            ProjectCommands::New { name, description, tags } => {
                let project = ProjectModel { 
                    id: None,
                    name,
                    description,
                    cumulative_time: 0};

                let tag_vec = tags.unwrap_or_else(Vec::new)
                    .into_iter()
                    .map(|tag_name| Tag {
                        id: None,
                        name: tag_name,
                    }).collect::<Vec<Tag>>();

                ProjectCommand::New { project, tags: tag_vec }

            },
            ProjectCommands::List => ProjectCommand::List,
        }
        
    }
    
}
#[derive(Subcommand, Clone)]
pub enum SessionCommands {
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
