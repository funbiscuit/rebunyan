use std::collections::VecDeque;
use std::io;
use std::io::Write;

use time::format_description::FormatItem;

use crate::log_entry::LogEntry;
use crate::log_level::LogLevel;
use crate::style::Style;
use crate::styled_writer::StyledWriter;

/// Maximum length of single entry in extras.
/// Otherwise it will be printed in details section.
const MAX_EXTRAS_LEN: usize = 50;

#[derive(Debug)]
pub struct EntryWriter {
    time_formatter: Vec<FormatItem<'static>>,
}

impl EntryWriter {
    pub fn write_formatted(
        &self,
        writer: &mut StyledWriter<impl Write>,
        entry: LogEntry,
    ) -> io::Result<()> {
        let hostname = if entry.hostname.is_empty() {
            "<no-hostname>"
        } else {
            entry.hostname.as_ref()
        };
        // not using write!(..) since it's a bit slower and makes harder to colorize output
        // without extra cost
        writer.write("[")?;
        writer.write_styled(
            &entry.time.format(&self.time_formatter).unwrap(),
            Style::White,
        )?;
        writer.write("] ")?;
        let (level, style) = Self::format_level(entry.level);
        writer.write_styled(&level, style)?;
        writer.write(": ")?;
        writer.write(&entry.name)?;
        writer.write("/")?;
        writer.write(&entry.pid.to_string())?;
        writer.write(" on ")?;
        writer.write(hostname)?;
        writer.write(": ")?;
        writer.write_styled(&entry.message, Style::Cyan)?;
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

    fn format_level(level: LogLevel) -> (String, Style) {
        match level {
            LogLevel::Trace => ("TRACE".to_owned(), Style::White),
            LogLevel::Debug => ("DEBUG".to_owned(), Style::Yellow),
            LogLevel::Info => (" INFO".to_owned(), Style::Cyan),
            LogLevel::Warn => (" WARN".to_owned(), Style::Magenta),
            LogLevel::Error => ("ERROR".to_owned(), Style::Red),
            LogLevel::Fatal => ("FATAL".to_owned(), Style::Inverse),
            LogLevel::Custom(level) => (format!("LVL{level}"), Style::None),
        }
    }

    fn write_leftover(
        mut writer: &mut StyledWriter<impl Write>,
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
            writer.write(" (")?;
            // reversing iterator because we pushed items to front
            (&mut writer, extras.iter().rev(), ", ").write_joined(
                |writer, (key, value, need_quotes)| {
                    writer.write_styled(key, Style::Bold)?;
                    writer.write("=")?;
                    if *need_quotes {
                        writer.write("\"")?;
                        writer.write(value)?;
                        writer.write("\"")
                    } else {
                        writer.write(value)
                    }
                },
            )?;
            writer.write(")")?;
        }

        (writer, details.iter(), "\n    --").write_joined(|writer, (key, value, _)| {
            writer.write("\n    ")?;
            writer.write_styled(key, Style::Bold)?;
            writer.write(": ")?;

            (writer, value.lines(), "\n    ").write_joined(|writer, line| writer.write(line))
        })?;
        Ok(())
    }
}

/// Helper trait to write items from iterator
/// without extra allocations
trait WriteJoined<W, T> {
    fn write_joined(
        &mut self,
        write_item: impl FnMut(&mut StyledWriter<W>, T) -> io::Result<()>,
    ) -> io::Result<()>;
}

impl<W, T, S, I> WriteJoined<W, T> for (S, I, &str)
where
    S: AsMut<StyledWriter<W>>,
    W: Write,
    I: Iterator<Item = T>,
{
    fn write_joined(
        &mut self,
        mut write_item: impl FnMut(&mut StyledWriter<W>, T) -> io::Result<()>,
    ) -> io::Result<()> {
        let (writer, iterator, delimiter) = self;
        if let Some(item) = iterator.next() {
            write_item(writer.as_mut(), item)?;
            for item in iterator {
                writer.as_mut().write(delimiter)?;
                write_item(writer.as_mut(), item)?;
            }
        }
        Ok(())
    }
}
