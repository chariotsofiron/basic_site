pub type Timestamp = i64;

#[derive(Default, Clone)]
pub struct AppState {}

/// Returns the current time in microseconds.
pub fn timestamp_micros() -> Timestamp {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_micros() as Timestamp
}

