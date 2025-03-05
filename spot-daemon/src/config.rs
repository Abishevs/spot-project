pub struct Config {
    pub socket_path: String,
}

impl Config {
    pub fn new(socket_path: &str) -> Self {
        Config {
            socket_path: socket_path.to_string(),
        }
    }
}
