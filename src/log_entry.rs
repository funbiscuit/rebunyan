use crate::log_level::LogLevel;
use serde::Deserialize;
use std::borrow::Cow;

/// Single entry in Bunyan format
#[derive(Debug, Deserialize)]
pub struct LogEntry<'a> {
    /// Bunyan log format version.
    /// The log version is a single integer:
    /// 0 is until version "1.0.0" of node-bunyan is released.
    /// Thereafter, starting with 1, this will be incremented
    /// if there is any backward incompatible change to the log record format.
    /// Details will be in "CHANGES.md" (of node-bunyan).
    #[serde(rename = "v")]
    pub version: u8,

    /// See `LogLevel`
    pub level: LogLevel,

    /// Name of the logger emitting logs that was provided at Logger creation.
    /// Typically this is the name of the service/app using Bunyan for logging
    #[serde(borrow)]
    pub name: Cow<'a, str>,

    /// Name of the operating system host `os.hostname()` if it was not overriden
    #[serde(borrow)]
    pub hostname: Cow<'a, str>,

    /// PID
    pub pid: u32,

    /// The date and time of the event in [ISO 8601 Extended Format](http://en.wikipedia.org/wiki/ISO_8601)
    /// and in UTC
    #[serde(with = "time::serde::iso8601")]
    pub time: time::OffsetDateTime,

    /// Log message.
    #[serde(rename = "msg", borrow)]
    pub message: Cow<'a, str>,

    /// All other, non required fields, that may be other nested objects
    #[serde(flatten)]
    pub leftover: serde_json::Map<String, serde_json::Value>,
}
