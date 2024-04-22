pub mod commands {
    use serde::{Serialize, Deserialize};

    use crate::models::{Project, Session, Tag};

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    pub enum MainCommand {
        Pomodoro(PomodoroCommand),
        Project(ProjectCommand),
        Session(SessionCommand),
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    pub enum PomodoroCommand {
        Start { duration: u64, break_time: u64 },
        Stop,
        Status,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    pub enum ProjectCommand {
        New { project: Project , tags: Vec<Tag>}, 
        Update { project: Project },
        Find { project: Project },
        List, // Returns projects
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    pub enum SessionCommand {
        Start { project: Project }, 
        End,
        Status,

    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    pub enum Response {
        Success(String),
        Error(String),
    }
}

pub mod models {
    use serde::{Serialize, Deserialize};
    use chrono::NaiveDateTime;

    #[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
    pub struct Project {
        pub id: Option<i64>, 
        pub name: String,
        pub cumulative_time: i64, // in Seconds?
        pub description: Option<String>,
    }

    #[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
    pub struct Session {
        pub id: Option<i64>,
        pub project_id: i64,
        pub start_time: NaiveDateTime,
        pub end_time: Option<NaiveDateTime>,
    }

    impl Session {
        pub fn status(&self) -> String {
            use chrono::Utc;
            let now = Utc::now().naive_utc();

            let duration = now.signed_duration_since(self.start_time);

            let days = duration.num_days();
            let hours = duration.num_hours() % 24;
            let minutes = duration.num_minutes() % 60;
            let seconds = duration.num_seconds() % 60;

            format!("{} days, {} hours, {} minutes, {} seconds", days, hours, minutes, seconds)
        }

        pub fn duration_in_seconds(&self) -> i64 {
            use chrono::Utc;

            let end_time = Utc::now().naive_utc();
            let duration = end_time - self.start_time;
            duration.num_seconds()
        }

    }

    #[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
    pub struct Tag {
        pub id: Option<i64>,
        pub name: String,
    }

    #[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
    pub struct ProjectTag {
        pub project_id: i64,
        pub tag_id: i64,
    }
}

pub mod utils {
    pub fn format_duration(seconds: i64) -> String {
        let hours = seconds as f64 / 3600.0;

        format!("{:.2} hours", hours)
    }
}

#[cfg(test)]
mod tests {
    use super::commands::*;
    use serde_json;

    #[test]
    fn test_serialize_response_success() {
        let response = Response::Success("Operation successful".to_string());
        let serialized = serde_json::to_string(&response).unwrap();
        assert_eq!(serialized, r#"{"Success":"Operation successful"}"#);
    }

    #[test]
    fn test_deserialize_response_success() {
        let data = r#"{"Success":"Operation completed"}"#;
        let response: Response = serde_json::from_str(data).unwrap();
        assert_eq!(response, Response::Success("Operation completed".to_string()));
    }

    #[test]
    fn test_serialize_response_error() {
        let response = Response::Error("An error occurred".to_string());
        let serialized = serde_json::to_string(&response).unwrap();
        assert_eq!(serialized, r#"{"Error":"An error occurred"}"#);
    }

    #[test]
    fn test_deserialize_response_error() {
        let data = r#"{"Error":"Failed to complete"}"#;
        let response: Response = serde_json::from_str(data).unwrap();
        assert_eq!(response, Response::Error("Failed to complete".to_string()));
    }
}

