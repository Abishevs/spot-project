extern crate spot_lib;

use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::os::unix::net::UnixStream;
use serde_json::Error;

use spot_lib::commands::{PomodoroCommand, Response};
use crate::notify::Notifier;
use crate::service::pomodoro::PomodoroService;
use crate::database::DbConnection;


pub struct CommandHandler<'a> {
    stream: UnixStream,
    db_connection: &'a DbConnection,
    notifier: &'a dyn Notifier,
}

impl<'a> CommandHandler<'a> {
    pub fn new(stream: UnixStream , db_connection: &'a DbConnection, notifier: &'a dyn Notifier) -> Self {
        CommandHandler {
            stream,
            db_connection,
            notifier,
        }
    }
    pub fn handle(&self) -> io::Result<()> {
        let reader = BufReader::new(&self.stream);
        let mut writer = BufWriter::new(&self.stream);

        for line in reader.lines() {
            let line = line?;
            let command: Result<PomodoroCommand, Error> = serde_json::from_str(&line);

            let response = match command {
                Ok(PomodoroCommand::Start) => {
                    self.notifier.send("Pomodoro", "Pomodor started"); 
                    PomodoroService::start(&self.db_connection);
                    serde_json::to_string(&Response::Success("Pomodoro started".to_string()))
                },
                Ok(PomodoroCommand::Stop) => {
                    self.notifier.send("Pomodoro", "Pomodor stoped"); 
                    PomodoroService::stop(&self.db_connection);
                    serde_json::to_string(&Response::Success("Pomodoro stopped".to_string()))
                },
                Ok(PomodoroCommand::Status) => {
                    let status = PomodoroService::status(&self.db_connection);
                    self.notifier.send("PomodoroOOO", &status); 
                    serde_json::to_string(&Response::Success(status))
                },
                Err(_) => {
                    serde_json::to_string(&Response::Error("Invalid command".to_string()))
                }
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
