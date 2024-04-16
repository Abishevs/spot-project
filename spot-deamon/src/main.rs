mod pomodoro;
extern crate spot_lib;

use std::os::unix::net::{UnixListener, UnixStream};
use std::io::{BufRead, BufReader, BufWriter, Write};
use dirs::home_dir;
use std::path::PathBuf;
use std::io;
use notify_rust::Notification;
use serde_json::Error;

use pomodoro::PomodoroTimer;
use spot_lib::commands::{PomodoroCommand, Response};

fn get_db_path() -> PathBuf {
    let mut path = home_dir().expect("Home directory not found");
    path.push(".config");
    path.push("spot");
    path.push("database.db");
    path
}

fn show_notification(body: &str) {
    Notification::new()
        .summary("S.P.O.T")
        .body(&body)
        .show().expect("Notfication failed");
}

fn handle_client(stream: UnixStream, timer: &mut PomodoroTimer) -> io::Result<()> {
    let reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    for line in reader.lines() {
        let line = line?;
        let command: Result<PomodoroCommand, Error> = serde_json::from_str(&line);

        let response = match command {
            Ok(PomodoroCommand::Start) => {
                show_notification("Pomodoro started sheesh");
                timer.start();
                serde_json::to_string(&Response::Success("Pomodoro started".to_string()))
            },
            Ok(PomodoroCommand::Stop) => {
                show_notification("Pomodoro stopped");
                timer.stop();
                serde_json::to_string(&Response::Success("Pomodoro stopped".to_string()))
            },
            Ok(PomodoroCommand::Status) => {
                let status = timer.status();
                show_notification(&status);
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

fn main() -> std::io::Result<()> {
    let path = get_db_path();
    println!("{:?}", path);
    let socket_path = "/tmp/spot-deamon.socket";
    std::fs::remove_file(socket_path).ok();  
    let listener = UnixListener::bind(socket_path)?;

    let mut timer = PomodoroTimer::new(25); 
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream, &mut timer)?;
            },
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }

    Ok(())
}

