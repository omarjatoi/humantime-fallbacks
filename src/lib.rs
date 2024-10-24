use std::{str::FromStr, time::Duration};

#[derive(Debug)]
enum TimeUnit {
    Microsecond,
    Nanosecond,
    Millisecond,
    Second,
    Minute,
    Hour,
    Day,
}

const SECONDS_IN_MINUTE: u64 = 60;
const SECONDS_IN_HOUR: u64 = 60 * SECONDS_IN_MINUTE;
const SECONDS_IN_DAY: u64 = 24 * SECONDS_IN_HOUR;

fn parse_duration(dur: &str, unit: TimeUnit) -> anyhow::Result<Duration> {
    if let Ok(duration) = humantime::Duration::from_str(dur) {
        return Ok(duration.into());
    }
    match unit {
        TimeUnit::Microsecond => {
            if let Ok(micros) = dur.parse::<u64>() {
                return Ok(std::time::Duration::from_micros(micros));
            }
        }
        TimeUnit::Nanosecond => {
            if let Ok(nanos) = dur.parse::<u64>() {
                return Ok(std::time::Duration::from_nanos(nanos));
            }
        }
        TimeUnit::Millisecond => {
            if let Ok(millis) = dur.parse::<u64>() {
                return Ok(std::time::Duration::from_millis(millis));
            }
        }
        TimeUnit::Second => {
            if let Ok(secs) = dur.parse::<u64>() {
                return Ok(std::time::Duration::from_secs(secs));
            }
        }
        TimeUnit::Minute => {
            if let Ok(mins) = dur.parse::<u64>() {
                return Ok(std::time::Duration::from_secs(mins * SECONDS_IN_MINUTE));
            }
        }
        TimeUnit::Hour => {
            if let Ok(hours) = dur.parse::<u64>() {
                return Ok(std::time::Duration::from_secs(hours * SECONDS_IN_HOUR));
            }
        }
        TimeUnit::Day => {
            if let Ok(days) = dur.parse::<u64>() {
                return Ok(std::time::Duration::from_secs(days * SECONDS_IN_DAY));
            }
        }
    }
    Err(anyhow::anyhow!(
        "Invalid duration: '{}' with unit: '{:?}'.",
        dur,
        unit
    ))
}

pub fn parse_duration_fallback_us(dur: &str) -> anyhow::Result<Duration> {
    parse_duration(dur, TimeUnit::Microsecond)
}

pub fn parse_duration_fallback_ns(dur: &str) -> anyhow::Result<Duration> {
    parse_duration(dur, TimeUnit::Nanosecond)
}

pub fn parse_duration_fallback_ms(dur: &str) -> anyhow::Result<Duration> {
    parse_duration(dur, TimeUnit::Millisecond)
}

pub fn parse_duration_fallback_sec(dur: &str) -> anyhow::Result<Duration> {
    parse_duration(dur, TimeUnit::Second)
}

pub fn parse_duration_fallback_min(dur: &str) -> anyhow::Result<Duration> {
    parse_duration(dur, TimeUnit::Minute)
}

pub fn parse_duration_fallback_hour(dur: &str) -> anyhow::Result<Duration> {
    parse_duration(dur, TimeUnit::Hour)
}

pub fn parse_duration_fallback_day(dur: &str) -> anyhow::Result<Duration> {
    parse_duration(dur, TimeUnit::Day)
}
