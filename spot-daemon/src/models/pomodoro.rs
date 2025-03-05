use std::time::{Duration, Instant};

#[derive(Debug, Copy, Clone)]
pub struct PomodoroTimer {
    duration: Duration,
    break_time: Duration,
    start_time: Option<Instant>,
}

impl PomodoroTimer {
    pub fn new(duration_in_min: u64, break_time: u64) -> Self {
        PomodoroTimer {
            duration: Duration::from_secs(duration_in_min * 60),
            break_time: Duration::from_secs(break_time * 60),
            start_time: None,
        }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn stop(&mut self) {
        self.start_time = None;
    }

    pub fn status(&self) -> String {
        match self.start_time {
            Some(start) => {
                let elapsed = start.elapsed();
                if elapsed >= self.duration {
                    "Pomodoro session completed".to_string()
                } else {
                    let remaining = self.duration - elapsed;
                    self.format_duration(remaining)
                }
            }
            None => "Pomodoro not started".to_string(),
        }
    }

    // Helper function to format duration
    fn format_duration(&self, duration: Duration) -> String {
        let total_seconds = duration.as_secs();
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;

        if hours > 0 {
            format!("{} hrs, {} min, {} sec remaining", hours, minutes, seconds)
        } else if minutes > 0 {
            format!("{} min, {} sec remaining", minutes, seconds)
        } else {
            format!("{} sec remaining", seconds)
        }
    }
}
