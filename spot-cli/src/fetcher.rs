use std::error::Error;
use std::io::{self, BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;

use spot_lib::commands::{MainCommand, Response};
use spot_lib::models;

use crate::picker::ProjectAdapter;

pub struct DaemonClient {
    stream: Option<UnixStream>,
    endpoint: String,
}

impl DaemonClient {
    pub fn new(endpoint: &str) -> Self {
        DaemonClient {
            stream: None,
            endpoint: endpoint.to_string(),
        }
    }

    pub fn connect(&mut self) -> Result<(), io::Error> {
        self.stream = Some(UnixStream::connect(&self.endpoint)?);
        Ok(())
    }

    // Takes in a shared MainCommand Enum which is serilizable
    pub fn send_command(&mut self, main_command: &MainCommand) -> Result<String, io::Error> {
        let command_json = serde_json::to_string(&main_command)?;
        // Dont take? Got problems with taking ownership... before
        // if let Some(mut stream) = self.stream.take() {
        if let Some(stream) = &mut self.stream {
            stream.write_all(command_json.as_bytes())?;
            stream.write_all(b"\n")?;

            let mut reader = BufReader::new(stream);
            let mut response = String::new();
            reader.read_line(&mut response)?;
            Ok(format!("{}", response.to_string()))
        } else {
            Ok(format!("ERROR: Failed reading from stream"))
        }
    }

    pub fn fetch_projects(&mut self) -> Result<Vec<ProjectAdapter>, io::Error> {
        // Placeholder: Mock data Fetching from the daemon
        let main_command = MainCommand::Project(spot_lib::commands::ProjectCommand::List);
        let res = self.send_command(&main_command).map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Error handling list command: {}", e),
            )
        })?;

        // println!("RES: {:?}", res);
        match serde_json::from_str::<Response>(&res) {
            Ok(Response::Success(data)) => {
                match serde_json::from_str::<Vec<models::Project>>(&data) {
                    Ok(projects) => {
                        println!("Projects: {:?}", projects);
                        // Convert models::Project to ProjectAdapter here
                        let project_adapters = projects
                            .into_iter()
                            .map(|proj| ProjectAdapter { project: proj })
                            .collect::<Vec<ProjectAdapter>>();
                        Ok(project_adapters)
                    }
                    Err(e) => Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("Error parsing projects: {}", e),
                    )),
                }
            }
            Ok(Response::Error(msg)) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error from server: {}", msg),
            )),
            Err(e) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error deserializing response: {}", e),
            )),
        }
    }
}
