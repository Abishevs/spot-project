pub mod commands {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    pub enum MainCommand {
        Pomodoro(PomodoroCommand),
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    pub enum PomodoroCommand {
        Start { duration: u64, break_time: u64 },
        Stop,
        Status,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    pub enum Response {
        Success(String),
        Error(String),
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

