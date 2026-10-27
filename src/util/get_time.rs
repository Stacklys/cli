use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_time() -> String {
  let duration = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("The system clock is ahead of the Unix epoch");

  let total_seconds = duration.as_secs();
  let seconds_in_day = total_seconds % 86400;

  let hours = seconds_in_day / 3600;
  let minutes = (seconds_in_day % 3600) / 60;
  let seconds = seconds_in_day % 60;

  format!("{}:{}:{}", hours, minutes, seconds)
}
