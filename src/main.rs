use std::io::{BufRead, BufWriter};

use clap::Parser;
use supports_color::Stream;

use crate::entry_writer::EntryWriter;
use crate::log_entry::LogEntry;
use crate::styled_writer::StyledWriter;

mod entry_writer;
mod log_entry;
mod log_level;
mod style;
mod styled_writer;

#[derive(Parser)]
#[command(about, author, long_about = None, version)]
struct Cli {
    /// Colorize output.
    /// If not set, will try to detect whether terminal supports coloring
    #[arg(long = "color", conflicts_with = "no_color")]
    color: bool,

    #[arg(long = "condition")]
    condition: Option<String>,

    /// Force no coloring
    #[arg(long = "no-color", conflicts_with = "color")]
    no_color: bool,
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
            formatter.write_formatted(&mut writer, entry).unwrap();
        } else {
            writer.write(&line).unwrap();
        }

        writer.write("\n").unwrap();
        writer.flush().unwrap();
    }
}
