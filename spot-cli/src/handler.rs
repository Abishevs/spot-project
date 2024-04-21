use spot_lib::commands::{MainCommand, PomodoroCommand, ProjectCommand};
use spot_lib::models;

use crate::{fetcher::DaemonClient, cli::{Config, ProjectCommands}, picker::show_picker_project};
use std::io;
use crate::cli::{PomodoroCommands, SessionCommands};

pub struct CommandHandler<'a> {
    daemon_client: &'a mut DaemonClient,
}

impl<'a> CommandHandler<'a> {
    pub fn new(daemon_client: &'a mut DaemonClient) -> Self {
        CommandHandler { daemon_client }
    }

    pub fn handle_pomodoro(&mut self, command: &PomodoroCommands) -> io::Result<String> {
        let res = match command {
            PomodoroCommands::Start { duration, break_time } => {
                println!("Starting Pomodoro for {} minutes, break after {} minutes", duration, break_time);
                let command = PomodoroCommand::from(command.clone());  
                let main_command = MainCommand::Pomodoro(command);
                match self.daemon_client.send_command(&main_command) {
                    Ok(res) => res,
                    Err(e) => format!("ERROR: handling pomodoro command: {}", e),

                }
            },
            PomodoroCommands::Stop => {
                println!("Stopping Pomodoro");
                let command = PomodoroCommand::from(command.clone());  
                let main_command = MainCommand::Pomodoro(command);
                match self.daemon_client.send_command(&main_command) {
                    Ok(res) => res,
                    Err(e) => format!("ERROR: handling pomodoro command: {}", e),

                }
            },
            PomodoroCommands::Status => {
                let command = PomodoroCommand::from(command.clone());  
                let main_command = MainCommand::Pomodoro(command);
                match self.daemon_client.send_command(&main_command) {
                    Ok(res) => {
                        println!("Current status: {}", &res);
                        res
                    },
                    Err(e) => format!("ERROR: handling pomodoro command: {}", e),

                }
            },
        };
        Ok(res)
    }

    pub fn handle_project(&mut self, command: &ProjectCommands) -> io::Result<String> {
        let res = match command {
            ProjectCommands::New { name, description, tags } => {
                println!("name: {:?}, descr: {:?} tags: {:?}", name, description, tags);
                let command = ProjectCommand::from(command.clone());  
                let main_command = MainCommand::Project(command);
                match self.daemon_client.send_command(&main_command) {
                    Ok(res) => res,
                    Err(e) => format!("ERROR: handling pomodoro command: {}", e),

                }
            },

            ProjectCommands::List => { 
                println!("Listing all projects");
                match self.daemon_client.fetch_projects() {
                    Ok(res) => format!("projects: {:?}", res),
                    Err(e) => format!("ERROR: {}", e),
                }

            }
        };
        Ok(res)
    }

    pub fn handle_session(&mut self, command: &SessionCommands) -> io::Result<String> {
        match command {
            SessionCommands::Start => {
                let projects = self.daemon_client.fetch_projects()?;
                let selected_project = show_picker_project(projects)?;
                match selected_project {
                    Some(project) => {
                        Ok(format!("selected project: {:?}", project))
                    },
                    None => Ok(format!("Project not found")),
                }

            }
            _ => Ok(format!("Not impl"))
        }
    }

    pub fn handle_config(&mut self, _command: &Config) -> io::Result<String> {
        Ok("Configured".to_string())
    }
}
