use crate::models::pomodoro::PomodoroTimer;
pub struct PomodoroService {
    pomodoro_timer: Option<PomodoroTimer>,
}

impl PomodoroService {
    pub fn new() -> Self {
        PomodoroService {
            pomodoro_timer: None,
        }
    }

    pub fn start(&mut self) {
        let timer = PomodoroTimer::new(25);
        self.pomodoro_timer = Some(timer);
        let timer = self.pomodoro_timer.as_mut().expect("PomodoroTimer must be started");
        timer.start();
        println!("Pomodoro started: {:?}", timer);
    }

    pub fn stop(&mut self) {
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
