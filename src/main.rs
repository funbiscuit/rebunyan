use std::io::{BufRead, BufWriter, Write};

use crate::entry_writer::EntryWriter;
use crate::log_entry::LogEntry;

mod entry_writer;
mod log_entry;
mod log_level;

fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let reader = stdin.lock();
    let mut writer = BufWriter::with_capacity(1024 * 1024, stdout.lock());
    let formatter = EntryWriter::new();
    for line in reader.lines() {
        let line = line.unwrap();

        if let Ok(entry) = serde_json::from_str::<LogEntry>(&line) {
            formatter.write_formatted(&mut writer, entry).unwrap();
        } else {
            writer.write_all(line.as_bytes()).unwrap();
        }

        writer.write_all(b"\n").unwrap();
        writer.flush().unwrap();
    }
}
