use std::ops::Add;
use std::time::SystemTime;
use chrono::{DateTime, Local, NaiveDateTime};

pub fn chrono2sys(c: NaiveDateTime) -> SystemTime {
    SystemTime::UNIX_EPOCH.add(core::time::Duration::from_millis(c.timestamp_millis() as u64))
}

pub fn time_fmt(t: DateTime<Local>) -> String {
    t.format("%Y-%m-%dT%H:%M").to_string()
}