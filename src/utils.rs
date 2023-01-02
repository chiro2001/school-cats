use std::ops::Add;
use std::time::SystemTime;
use chrono::NaiveDateTime;

pub fn chrono2sys(c: NaiveDateTime) -> SystemTime {
    SystemTime::UNIX_EPOCH.add(core::time::Duration::from_millis(c.timestamp_millis() as u64))
}