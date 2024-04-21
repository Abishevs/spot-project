mod notify;
mod handler;
mod service;
mod database;
mod models;
mod utils;
mod config;

use std::sync::Arc;
use std::io::{self, ErrorKind};
use service::api::ApiService;
use spot_lib::models::Project;
use utils::create_config_dir;

use crate::utils::get_db_path;

fn main() -> std::io::Result<()> {

    let config = config::Config::new("/tmp/spot-daemon.socket");
    let config_path = match create_config_dir() {
        Ok(path) => path,
        Err(e) => return Err(e),
    };

    let db_connection = match database::DbConnection::new(&config_path) {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Failed to initialize database: {}", e);
            return Err(io::Error::new(ErrorKind::Other, "Failed to initialize database"));
        }
    };

    //Test db
    let project = Project{ 
        id: None,
        name: "projecting 1".to_string(),
        cumulative_time: 0,
        description: Some("This is my first test project".to_string()),
    };

    let project1 = Project{ 
        id: None,
        name: "Projecting 1".to_string(),
        cumulative_time: 0,
        description: Some("This is my second test project".to_string()),
    };
    let res = db_connection.create_project(&project);
    match res {
        Ok(_) => println!("Project created successfully."),
        Err(e) => println!("Failed to create project: {}", e),
    }
    let res = db_connection.create_project(&project1);
    match res {
        Ok(_) => println!("Project created successfully."),
        Err(e) => println!("Failed to create project: {}", e),
    }
    match db_connection.list_projects() {
        Ok(projects) if projects.is_empty() => println!("No projects found."),
        Ok(projects) => println!("Projects listed: {:?}", projects),
        Err(e) => eprintln!("Failed to list projects: {}", e),
    }
    // clean up db
    let _ = std::fs::remove_file(get_db_path(&config_path)).ok();  
    // clean up 
    let _ = std::fs::remove_file(&config.socket_path).ok();  

    // Setup components
    let notifier = Arc::new(notify::DesktopNotifier);
    let listener = service::api::UnixSocketService::start_listener(&config.socket_path)?;
    let mut handler = handler::CommandHandler::new(&db_connection, notifier);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handler.handle(stream)?,
            Err(e) => eprintln!("ERROR handling unix stream {}", e),
        }
    }
    Ok(())
}

