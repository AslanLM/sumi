use std::time::SystemTime;

#[derive(Debug, Clone)]
pub enum LogSource {
    Stdout,
    Stderr,
}

#[derive(Debug, Clone)]
pub struct LogEvent {
    pub source: LogSource,
    pub message: String,
    pub timestamp: SystemTime,
}
