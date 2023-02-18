use serde::{Deserialize, Deserializer};

#[derive(Debug, Eq, PartialEq)]
pub enum LogLevel {
    /// The service/app is going to stop or become unusable now.
    /// An operator should definitely look into this soon
    Fatal,

    /// Fatal for a particular request, but the service/app continues servicing other requests.
    /// An operator should look at this soon(ish)
    Error,

    /// A note on something that should probably be looked at by an operator eventually
    Warn,

    /// Detail on regular operation
    Info,

    /// Anything else, i.e. too verbose to be included in "info" level
    Debug,

    /// Logging from external libraries used by your app or very detailed application logging
    Trace,

    /// Custom level, that will be formatted like LVL%d
    Custom(u8),
}

impl<'de> Deserialize<'de> for LogLevel {
    fn deserialize<D>(deserializer: D) -> Result<LogLevel, D::Error>
    where
        D: Deserializer<'de>,
    {
        let level = match u8::deserialize(deserializer)? {
            10 => LogLevel::Trace,
            20 => LogLevel::Debug,
            30 => LogLevel::Info,
            40 => LogLevel::Warn,
            50 => LogLevel::Error,
            60 => LogLevel::Fatal,
            level => LogLevel::Custom(level),
        };

        Ok(level)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::log_level::LogLevel;

    #[rstest]
    #[case("15", LogLevel::Custom(15))]
    #[case("35", LogLevel::Custom(35))]
    #[case("10", LogLevel::Trace)]
    #[case("20", LogLevel::Debug)]
    #[case("30", LogLevel::Info)]
    #[case("40", LogLevel::Warn)]
    #[case("50", LogLevel::Error)]
    #[case("60", LogLevel::Fatal)]
    fn test_deserialize(#[case] input: &str, #[case] expected: LogLevel) {
        let level: LogLevel = serde_json::from_str(input).unwrap();
        assert_eq!(level, expected);
    }
}
