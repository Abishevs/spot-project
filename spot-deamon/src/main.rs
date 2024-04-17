use service::api::ApiService;

mod notify;
mod handler;
mod service;
mod database;
mod models;
mod utils;
mod config;

fn main() -> std::io::Result<()> {
    let config = config::Config::new("/tmp/spot-deamon.socket");
    let db_connection = database::DbConnection::new();
    let notifier = notify::DesktopNotifier;
    std::fs::remove_file(&config.socket_path).ok();  
    let listener = service::api::UnixSocketService::start_listener(&config.socket_path)?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handler::CommandHandler::new(stream, &db_connection, &notifier).handle()?,
            Err(e) => eprintln!("ERROR handling unix stream {}", e),
        }
    }
    Ok(())
}

