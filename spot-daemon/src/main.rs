mod notify;
mod handler;
mod service;
mod database;
mod models;
mod utils;
mod config;

use std::sync::Arc;
use service::api::ApiService;

fn main() -> std::io::Result<()> {

    let config = config::Config::new("/tmp/spot-daemon.socket");
    let db_connection = database::DbConnection::new();
    let notifier = Arc::new(notify::DesktopNotifier);
    std::fs::remove_file(&config.socket_path).ok();  
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

