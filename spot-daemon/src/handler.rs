extern crate spot_lib;

use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::os::unix::net::UnixStream;
use std::sync::Arc;
use serde_json::Error;

use spot_lib::commands::{MainCommand, PomodoroCommand, ProjectCommand, Response, SessionCommand};
use crate::notify::Notifier;
use crate::service::pomodoro::PomodoroService;
use crate::database::DbConnection;

pub trait Command {
    fn execute(&self, handler: &mut CommandHandler) -> io::Result<String>;
}

impl Command for MainCommand {
    fn execute(&self, handler: &mut CommandHandler) -> io::Result<String> {
        match self {
            MainCommand::Pomodoro(cmd) => cmd.execute(handler),
            MainCommand::Project(cmd) => cmd.execute(handler),
            MainCommand::Session(cmd) => cmd.execute(handler),
        }
    }
}

impl Command for SessionCommand {
    fn execute(&self, handler: &mut CommandHandler) -> io::Result<String> {
        match self {
            SessionCommand::Start { project } => {
                println!("HELLOOO=???");
                let msg = format!("Started lets goo\n Project: {:?}", project);
                handler.notifier.send("Session",
                                      &msg.to_string());
                Ok(msg)
                // unimplemented!()
            },
            SessionCommand::Stop { project } => {!unimplemented!()},
        }
    }
    
}

impl Command for ProjectCommand {
    fn execute(&self, handler: &mut CommandHandler) -> io::Result<String> {
        match self {
            ProjectCommand::New { project , tags} => {
                match handler.db_connection.create_project_with_tags(project.clone(), tags.to_vec()) {
                    Ok(res) => Ok(res),
                    Err(e) => Err(e),
                }
            },

            ProjectCommand::List => {
                match handler.db_connection.list_projects() {
                    Ok(projects) if projects.is_empty() => Err(io::Error::new(io::ErrorKind::NotFound,
                                                                              format!("No projects found."))),
                    Ok(projects) => {
                        println!("Projects listed: {:?}", &projects);
                        Ok(serde_json::to_string(&projects)?)
                    },
                    Err(e) => Err(io::Error::new(io::ErrorKind::Other, 
                                                 format!("Failed to list projects: {}",
                                                         e))),
                }
            },
            ProjectCommand::Find { project } => {
                unimplemented!()
            },
            ProjectCommand::Update { project } => {
                unimplemented!()
            }
        }
    }
}

impl Command for PomodoroCommand {
    fn execute(&self, handler: &mut CommandHandler) -> io::Result<String> {
        match self {
            PomodoroCommand::Start { duration, break_time } => {
                let res = handler.pomodoro_service.start(*duration, *break_time);
                handler.notifier.send("Pomodoro", &res);
                Ok(res)
            },
            PomodoroCommand::Stop => {
                handler.notifier.send("Pomodoro", "Pomodoro stopped");
                handler.pomodoro_service.stop();
                Ok("Pomodoro stopped".to_string())
            },
            PomodoroCommand::Status => {
                let status = handler.pomodoro_service.status();
                handler.notifier.send("Pomodoro", &status);
                Ok(status)
            },
        }
    }
}

pub struct CommandHandler<'a> {
    db_connection: &'a mut DbConnection,
    notifier: Arc<dyn Notifier + Send + Sync>,
    pomodoro_service: PomodoroService,
}


impl<'a> CommandHandler<'a> {
    pub fn new(db_connection: &'a mut DbConnection,
               notifier: Arc<(dyn Notifier + Send + Sync)>) -> Self {
        let pomodoro_service = PomodoroService::new(Arc::clone(&notifier));
        CommandHandler {
            db_connection,
            notifier,
            pomodoro_service,
        }
    }

    pub fn handle(&mut self, stream: UnixStream) -> io::Result<()> {
        let reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);

        for line in reader.lines() {
            let line = line?;
            let command: Result<MainCommand, Error> = serde_json::from_str(&line);
            println!("{}", &line.to_string());

            let response = match command {
                Ok(cmd) => {
                    match cmd.execute(self) {
                        Ok(msg) => serde_json::to_string(&Response::Success(msg)),
                        Err(e) => serde_json::to_string(&Response::Error(e.to_string())),
                    }
                },
                Err(_) => serde_json::to_string(&Response::Error("Invalid command".to_string())),
            };

            if let Ok(json) = response {
                writer.write_all(json.as_bytes())?;
                writer.write_all(b"\n")?;
                writer.flush()?;
            }
        }
        Ok(())
    }
}
