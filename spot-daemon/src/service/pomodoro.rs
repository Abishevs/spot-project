use std::{thread, u64};
use std::time::{Duration, Instant};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

use crate::notify::{DesktopNotifier, SoundCapable, SoundType};
use crate::{models::pomodoro::PomodoroTimer, notify::Notifier};
pub struct PomodoroService {
    notifier: Arc<dyn Notifier + Send + Sync>,
    pomodoro_timer: Option<PomodoroTimer>,
    is_running: Arc<AtomicBool>,
}

impl PomodoroService {
    pub fn new(notifier: Arc<dyn Notifier + Send + Sync>) -> Self {
        PomodoroService {
            notifier,
            pomodoro_timer: None,
            is_running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start(&mut self, duration: u64, break_time: u64) -> String {
        if let Some(timer) = self.pomodoro_timer {
            return format!("Timer is running: \n Status: {}", timer.status());
        }

        let timer = PomodoroTimer::new(duration, break_time);
        self.pomodoro_timer = Some(timer);
        let timer = self.pomodoro_timer.as_mut().expect("PomodoroTimer must be started");
        timer.start();

        self.is_running.store(true, Ordering::SeqCst);
        let is_running = Arc::clone(&self.is_running);
        let notifier = Arc::clone(&self.notifier);

        thread::spawn(move || {
            // Start session timer
            let start = Instant::now();
            let duration = Duration::from_secs(duration * 60);

            while start.elapsed() < duration {
                if !is_running.load(Ordering::SeqCst) {
                    println!("Timer stopped.");
                    return;
                }
                thread::sleep(Duration::from_secs(1));
            }

            notifier.send("Pomodoro", "Time's up! Take a break.");
            if let Some(sound_notifier) = notifier.as_any().downcast_ref::<DesktopNotifier>() {
                sound_notifier.play_sound(SoundType::PomodoroEnd);
            }

            // Start break Timer
            let break_start = Instant::now();
            let break_duration = Duration::from_secs(break_time * 60);

            thread::sleep(Duration::from_secs(5));

            notifier.send("Pomdoro", "Break started...");
            if let Some(sound_notifier) = notifier.as_any().downcast_ref::<DesktopNotifier>() {
                sound_notifier.play_sound(SoundType::BreakStart);
            }

            while break_start.elapsed() < break_duration {
                if !is_running.load(Ordering::SeqCst) {
                    println!("Timer stopped.");
                    return;
                }
                thread::sleep(Duration::from_secs(1));
            }
            
            
            notifier.send("Pomdoro", "Break Ended...");
            if let Some(sound_notifier) = notifier.as_any().downcast_ref::<DesktopNotifier>() {
                sound_notifier.play_sound(SoundType::BreakEnd);
            }
            
            is_running.store(false, Ordering::SeqCst);
        });
        format!("Pomodoro started")

}

    pub fn stop(&mut self) {
        if self.pomodoro_timer.is_some() {
            self.is_running.store(false, Ordering::SeqCst);
            self.pomodoro_timer = None;
            println!("Pomodoro stopped.");
        } else {
            println!("No timer is running.");
        }
    }

    pub fn pause(&mut self) {
        if let Some(timer) = self.pomodoro_timer.as_mut() {
            timer.stop();
            println!("Pomodoro stopped: {:?}", timer);
        }
    }

    pub fn status(&self) -> String {
        if let Some(timer) = self.pomodoro_timer {
            let status = timer.status();
            println!("STATUS: {}", status);
            timer.status()
        } else {
            "No active Pomodoro".to_string()
        }
    }
}
