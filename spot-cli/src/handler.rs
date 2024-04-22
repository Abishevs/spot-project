use spot_lib::commands::{self, MainCommand, PomodoroCommand, ProjectCommand};
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
                    Err(e) => format!("ERROR: handling project command: {}", e),

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

    pub fn handle_session(&mut self, cmd: &SessionCommands) -> io::Result<String> {
        let res = match cmd {
            SessionCommands::Start => {
                let project_adapters = self.daemon_client.fetch_projects()?;
                let selected_project_adapter = show_picker_project(project_adapters)?;
                match selected_project_adapter {
                    Some(project) => {
                        let command = commands::SessionCommand::Start { project: project.project.clone() };
                        let main_command = MainCommand::Session(command);
                        println!("DEBUG: {:?}", main_command);
                        match self.daemon_client.send_command(&main_command) {
                            Ok(res) => res,
                            Err(e) => format!("ERROR: handling session command: {}", e),
                            // Ok(format!("selected project: {:?}", project))
                        }
                    },
                    None => format!("Project not found"),
                }

            },
            SessionCommands::End => {
                let cmd= commands::SessionCommand::End;
                let main_cmd = MainCommand::Session(cmd);
                match self.daemon_client.send_command(&main_cmd) {
                    Ok(res) => res,
                    Err(e) => format!("ERROR: handling session command: {}", e),
                }
            },
            SessionCommands::Status => {
                let cmd= commands::SessionCommand::Status;
                let main_cmd = MainCommand::Session(cmd);
                match self.daemon_client.send_command(&main_cmd) {
                    Ok(res) => res,
                    Err(e) => format!("ERROR: handling session command: {}", e),
                }
            }
        };
        Ok(res)
    }

    pub fn handle_config(&mut self, _command: &Config) -> io::Result<String> {
        Ok("Configured".to_string())
    }
}
