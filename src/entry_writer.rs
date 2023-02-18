use std::collections::VecDeque;
use std::io;
use std::io::Write;

use time::format_description::FormatItem;

use crate::log_entry::LogEntry;
use crate::log_level::LogLevel;

/// Maximum length of single entry in extras.
/// Otherwise it will be printed in details section.
const MAX_EXTRAS_LEN: usize = 50;

#[derive(Debug)]
pub struct EntryWriter {
    time_formatter: Vec<FormatItem<'static>>,
}

impl EntryWriter {
    pub fn write_formatted(&self, writer: &mut impl Write, entry: LogEntry) -> io::Result<()> {
        let hostname = if entry.hostname.is_empty() {
            "<no-hostname>"
        } else {
            entry.hostname.as_ref()
        };
        // not using write!(..) since it's a bit slower and makes harder to colorize output
        // without extra cost
        writer.write_all(b"[")?;
        writer.write_all(entry.time.format(&self.time_formatter).unwrap().as_bytes())?;
        writer.write_all(b"] ")?;
        writer.write_all(Self::write_level(entry.level).as_bytes())?;
        writer.write_all(b": ")?;
        writer.write_all(entry.name.as_bytes())?;
        writer.write_all(b"/")?;
        writer.write_all(entry.pid.to_string().as_bytes())?;
        writer.write_all(b" on ")?;
        writer.write_all(hostname.as_bytes())?;
        writer.write_all(b": ")?;
        writer.write_all(entry.message.as_bytes())?;
        Self::write_leftover(writer, entry.leftover)?;
        Ok(())
    }

    pub fn new() -> Self {
        let time_formatter = time::format_description::parse(
            "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3]\
            [offset_hour sign:mandatory]:[offset_minute]",
        )
        .unwrap();

        Self { time_formatter }
    }

    fn write_level(level: LogLevel) -> String {
        match level {
            LogLevel::Trace => "TRACE".to_owned(),
            LogLevel::Debug => "DEBUG".to_owned(),
            LogLevel::Info => " INFO".to_owned(),
            LogLevel::Warn => " WARN".to_owned(),
            LogLevel::Error => "ERROR".to_owned(),
            LogLevel::Fatal => "FATAL".to_owned(),
            LogLevel::Custom(level) => format!("LVL{level}"),
        }
    }

    fn write_leftover(
        mut writer: impl Write,
        extra_fields: serde_json::Map<String, serde_json::Value>,
    ) -> io::Result<()> {
        // items will be [extras...; details]
        let mut items = VecDeque::with_capacity(extra_fields.len());
        let mut extras_count = 0;

        for (key, value) in extra_fields {
            let (value, need_quotes) = match value {
                serde_json::Value::String(s) => {
                    // Wrap string in quotes if it is contains spaces or empty (and is oneliner)
                    let need_quotes = (s.contains(' ') && !s.contains('\n')) || s.is_empty();
                    (s, need_quotes)
                }
                value => (serde_json::to_string_pretty(&value).unwrap(), false),
            };

            if value.len() > MAX_EXTRAS_LEN || value.contains('\n') {
                // details (never wrapped in quotes)
                items.push_back((key, value, false));
            } else {
                // extras
                // pushing to front makes extras reversed
                items.push_front((key, value, need_quotes));
                extras_count += 1;
            }
        }

        let (extras, details) = items.make_contiguous().split_at(extras_count);
        if !extras.is_empty() {
            writer.write_all(b" (")?;
            // reversing iterator because we pushed items to front
            (&mut writer, extras.iter().rev(), ", ").write_joined(
                |writer, (key, value, need_quotes)| {
                    writer.write_all(key.as_bytes())?;
                    writer.write_all(b"=")?;
                    if *need_quotes {
                        writer.write_all(b"\"")?;
                        writer.write_all(value.as_bytes())?;
                        writer.write_all(b"\"")
                    } else {
                        writer.write_all(value.as_bytes())
                    }
                },
            )?;
            writer.write_all(b")")?;
        }

        (&mut writer, details.iter(), "\n    --").write_joined(|writer, (key, value, _)| {
            writer.write_all(b"\n    ")?;
            writer.write_all(key.as_bytes())?;
            writer.write_all(b": ")?;

            (writer, value.lines(), "\n    ")
                .write_joined(|writer, line| writer.write_all(line.as_bytes()))
        })?;
        Ok(())
    }
}

/// Helper trait to write items from iterator
/// without extra allocations
trait WriteJoined<W, T> {
    fn write_joined(self, write_item: impl FnMut(&mut W, T) -> io::Result<()>) -> io::Result<()>;
}

impl<W, T, I> WriteJoined<W, T> for (&mut W, I, &str)
where
    W: Write,
    I: Iterator<Item = T>,
{
    fn write_joined(
        self,
        mut write_item: impl FnMut(&mut W, T) -> io::Result<()>,
    ) -> io::Result<()> {
        let (writer, mut iterator, delimiter) = self;
        if let Some(item) = iterator.next() {
            write_item(writer, item)?;
            for item in iterator {
                writer.write_all(delimiter.as_bytes())?;
                write_item(writer, item)?;
            }
        }
        Ok(())
    }
}
