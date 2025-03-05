use std::io;
use std::os::unix::net::UnixListener;

pub trait ApiService {
    type Listener;

    fn start_listener(url: &str) -> io::Result<Self::Listener>;
}

pub struct UnixSocketService;

impl ApiService for UnixSocketService {
    type Listener = UnixListener;

    fn start_listener(socket_path: &str) -> io::Result<Self::Listener> {
        std::fs::remove_file(&socket_path).ok();
        let listener = UnixListener::bind(&socket_path)?;
        Ok(listener)
    }
}
