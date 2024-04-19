use tokio::time::{self, Duration};
use std::sync::Arc;

use crate::{models::pomodoro::PomodoroTimer, notify::Notifier};
pub struct PomodoroService {
    notifier: Arc<(dyn Notifier + Send + Sync)>,
    pomodoro_timer: Option<PomodoroTimer>,
}

impl PomodoroService {
    pub fn new(notifier: Arc<(dyn Notifier + Send + Sync)>) -> Self {
        PomodoroService {
            notifier,
            pomodoro_timer: None,
        }
    }

    pub fn start(&mut self, duration: u64, break_time: u64) -> String {
        match self.pomodoro_timer {
            Some(timer) => {
                format!("Timer is running: \n Status: {}", timer.status())
            },
            None => {
                let timer = PomodoroTimer::new(duration, break_time);
                self.pomodoro_timer = Some(timer);
                let timer = self.pomodoro_timer.as_mut().expect("PomodoroTimer must be started");
                timer.start();
                let notifier = handl;
                tokio::spawn(async move {
                    PomodoroService::run_timer(duration, break_time, notifier);
                });
                format!("Pomodoro started")
                
            },
        }
    }

    async fn run_timer(duration: u64, break_time: u64, notifier: Arc<dyn Notifier + Send + Sync>) {
        let sleep_duration = Duration::from_secs(duration * 60);
        time::sleep(sleep_duration).await;
        notifier.send("Pomodoro", "Time's up! Take a break.").await;
    }

    pub fn stop(&mut self) {
        self.pomodoro_timer = None;
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
