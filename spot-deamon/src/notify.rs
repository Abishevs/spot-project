use dirs::home_dir;
use std::process::Command;
use std::any::Any;

use notify_rust::Notification;

pub enum SoundType {
    BreakStart,
    BreakEnd,
    PomodoroEnd,
}

fn get_sound_path(sound: SoundType) -> String {
    let home_directory = home_dir().expect("Home directory not found");
    let base_path = home_directory.join(".local/share/spot/sounds/");
    let sound_file = match sound {
        SoundType::BreakStart => "break_start.mp3",
        SoundType::BreakEnd => "break_end.mp3",
        SoundType::PomodoroEnd => "pomodoro_end.mp3",
    };
    base_path.join(sound_file).to_str().unwrap().to_string()
}

pub trait Notifier: Send + Sync {
    fn send(&self, title: &str, message: &str);
    fn as_any(&self) -> &dyn Any; // Downcasting 
}

pub trait SoundCapable: Send + Sync {
    fn play_sound(&self, sound: SoundType);
}

pub struct DesktopNotifier;

impl Notifier for DesktopNotifier {
    fn send(&self, title: &str, message: &str) {
        Notification::new()
            .summary(title)
            .body(&message)
            .show().expect("Notfication send failed");
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    
}

impl SoundCapable for DesktopNotifier {
    fn play_sound(&self, sound: SoundType) {
        let path = get_sound_path(sound);
        println!("sound-PATH: {}", &path);
        let extension = std::path::Path::new(&path)
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or("");

        match extension {
            "wav" => {
                let _ = Command::new("aplay")
                    .arg(path)
                    .output();
            },
            "mp3" => {
                let res = Command::new("mpg123")
                    .arg(path)
                    .output();
                println!("RES: {:?}", &res);
            },
            _ => eprintln!("Unsupported audio format"),
        }
    }
}

