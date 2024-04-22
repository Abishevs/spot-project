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
use utils::{create_config_dir, get_db_path};

fn main() -> std::io::Result<()> {
    let config = config::Config::new("/tmp/spot-daemon.socket");
    let config_path = match create_config_dir() {
        Ok(path) => path,
        Err(e) => return Err(e),
    };

    let mut db_connection = match database::DbConnection::new(&config_path) {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Failed to initialize database: {}", e);
            return Err(io::Error::new(ErrorKind::Other, "Failed to initialize database"));
        }
    };

    // clean up db
    // let _ = std::fs::remove_file(get_db_path(&config_path)).ok();  
    // clean up 
    let _ = std::fs::remove_file(&config.socket_path).ok();  

    // Setup components
    let notifier = Arc::new(notify::DesktopNotifier);
    let listener = service::api::UnixSocketService::start_listener(&config.socket_path)?;
    let mut handler = handler::CommandHandler::new(&mut db_connection, notifier);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handler.handle(stream)?,
            Err(e) => eprintln!("ERROR handling unix stream {}", e),
        }
    }
    Ok(())
}

