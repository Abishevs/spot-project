use crate::database::DbConnection;
use crate::models::pomodoro::PomodoroTimer;
pub struct PomodoroService;

impl PomodoroService {
    pub fn start(db_connection: &DbConnection) {
        let mut timer = PomodoroTimer::new(25 * 60);  // 25 minutes
        timer.start();
        db_connection.save_timer(&timer);
        println!("Pomodoro started: {:?}", timer);
    }

    pub fn stop(db_connection: &DbConnection) {
        if let Some(mut timer) = db_connection.get_current_timer() {
            timer.stop();
            db_connection.update_timer(&timer);
            println!("Pomodoro stopped: {:?}", timer);
        }
    }

    pub fn status(db_connection: &DbConnection) -> String {
        if let Some(timer) = db_connection.get_current_timer() {
            timer.status()
        } else {
            "No active Pomodoro".to_string()
        }
    }
}
