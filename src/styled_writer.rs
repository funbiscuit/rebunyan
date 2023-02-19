use crate::style::Style;
use std::io;
use std::io::Write;

pub struct StyledWriter<W> {
    writer: W,
    stylize: bool,
}

impl<W: Write> StyledWriter<W> {
    pub fn new(writer: W, stylize: bool) -> Self {
        Self { writer, stylize }
    }

    pub fn write(&mut self, text: &str) -> io::Result<()> {
        self.writer.write_all(text.as_bytes())
    }

    pub fn write_styled(&mut self, text: &str, style: Style) -> io::Result<()> {
        if self.stylize {
            let (begin, end) = style.codes();
            self.write(begin)?;
            self.write(text)?;
            self.write(end)
        } else {
            self.write(text)
        }
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl<W> AsMut<StyledWriter<W>> for &mut StyledWriter<W> {
    fn as_mut(&mut self) -> &mut StyledWriter<W> {
        self
    }
}
