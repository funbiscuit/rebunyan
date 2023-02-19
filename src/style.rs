/// https://en.wikipedia.org/wiki/ANSI_escape_code#SGR_(Select_Graphic_Rendition)_parameters
#[derive(Debug, Eq, PartialEq)]
pub enum Style {
    None,
    Bold,
    Inverse,
    White,
    Cyan,
    Magenta,
    Red,
    Yellow,
}

impl Style {
    pub fn codes(&self) -> (&str, &str) {
        match self {
            Style::None => ("", ""),
            Style::Bold => ("\x1b[1m", "\x1b[22m"),
            Style::Inverse => ("\x1b[7m", "\x1b[27m"),
            Style::White => ("\x1b[37m", "\x1b[39m"),
            Style::Cyan => ("\x1b[36m", "\x1b[39m"),
            Style::Magenta => ("\x1b[35m", "\x1b[39m"),
            Style::Red => ("\x1b[31m", "\x1b[39m"),
            Style::Yellow => ("\x1b[33m", "\x1b[39m"),
        }
    }
}
