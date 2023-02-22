use std::io::{BufRead, BufWriter};

use clap::Parser;
use supports_color::Stream;

use crate::entry_writer::EntryWriter;
use crate::log_entry::LogEntry;
use crate::log_level::LogLevel;
use crate::styled_writer::StyledWriter;

mod entry_writer;
mod log_entry;
mod log_level;
mod style;
mod styled_writer;

#[derive(Parser)]
#[command(about, author, long_about = None, version)]
struct Cli {
    /// Only show messages after specified timestamp (in UTC).
    /// If date is given, show messages after 00:00:00 of this date.
    #[arg(long, value_parser = parse_after)]
    after: Option<time::OffsetDateTime>,

    /// Only show messages before specified timestamp (in UTC).
    /// If date is given, show messages before 23:59:59 of this date.
    #[arg(long, value_parser = parse_before)]
    before: Option<time::OffsetDateTime>,

    /// Colorize output.
    /// If not set, will try to detect whether terminal supports coloring
    #[arg(long, conflicts_with = "no_color")]
    color: bool,

    /// Only show messages at or above the specified level.
    /// Supported values: trace, debug, info, warn, error, fatal
    #[arg(short, long, default_value = "trace")]
    level: LogLevel,

    /// Force no coloring
    #[arg(long = "no-color", conflicts_with = "color")]
    no_color: bool,
}

fn parse_after(arg: &str) -> Result<time::OffsetDateTime, String> {
    parse_time(arg, time::Time::from_hms(0, 0, 0).unwrap())
}

fn parse_before(arg: &str) -> Result<time::OffsetDateTime, String> {
    parse_time(
        arg,
        time::Time::from_hms(0, 0, 0).unwrap() - time::Duration::nanoseconds(1),
    )
}

fn parse_time(arg: &str, default_time: time::Time) -> Result<time::OffsetDateTime, String> {
    let format =
        time::format_description::parse("[year]-[month]-[day]T[hour]:[minute]:[second]").unwrap();

    for i in 0..(format.len() - 1) {
        // gradually decrease amount of items we try to restore from give input
        let format = &format[0..(format.len() - i)];

        // try to parse as time
        if let Ok(time) = time::PrimitiveDateTime::parse(arg, format) {
            return Ok(time.assume_utc());
        }

        // try to parse as date
        if let Ok(date) = time::Date::parse(arg, format) {
            return Ok(date.with_time(default_time).assume_utc());
        }
    }

    Err(format!("Unable to parse '{arg}' as timestamp"))
}

fn main() {
    let cli = Cli::parse();
    // Stylize output if user chose it explicitly or terminal supports it
    let stylize = !cli.no_color && (cli.color || supports_color::on(Stream::Stdout).is_some());

    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let reader = stdin.lock();
    let writer = BufWriter::with_capacity(1024 * 1024, stdout.lock());
    let mut writer = StyledWriter::new(writer, stylize);
    let formatter = EntryWriter::new();
    for line in reader.lines() {
        let line = line.unwrap();

        if let Ok(entry) = serde_json::from_str::<LogEntry>(&line) {
            if entry.level < cli.level {
                continue;
            }
            if let Some(true) = cli.after.map(|t| entry.time < t) {
                continue;
            }
            if let Some(true) = cli.before.map(|t| entry.time > t) {
                continue;
            }
            formatter.write_formatted(&mut writer, entry).unwrap();
        } else {
            writer.write(&line).unwrap();
        }

        writer.write("\n").unwrap();
        writer.flush().unwrap();
    }
}
