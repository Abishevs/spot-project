use std::os::unix::net::UnixStream;
use std::io::{Write, BufRead, BufReader, self};
use std::error::Error;

use spot_lib::commands::MainCommand;

use crate::picker::Project;

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
    pub fn send_command(&mut self, main_command: &MainCommand) -> Result<String, io::Error>{
        let command_json = serde_json::to_string(&main_command)?;
        if let Some(mut stream) = self.stream.take() {
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

    pub fn fetch_projects(&self) -> Result<Vec<Project>, io::Error> {
        // Placeholder: Mock data Fetching from the daemon
        let projects = vec![
            Project { 
                name: "project1".to_string(),
                description: "descr1".to_string(),
            },
            Project { 
                name: "project2".to_string(),
                description: "descr2".to_string(),
            },

            Project { 
                name: "project3".to_string(),
                description: "descr3".to_string(),
            },
        ];
        Ok(projects)
    }
}
