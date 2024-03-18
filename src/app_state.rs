use std::time::SystemTime;

pub type Timestamp = i64;

#[derive(Default, Clone)]
pub struct AppState {}

/// Returns the current time in microseconds.
#[allow(clippy::cast_possible_truncation)]
pub fn timestamp_micros() -> Timestamp {
    SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_micros() as Timestamp
}
