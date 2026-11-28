//! # style
//! Items for styling widgets

use std::fmt;

/// # Style
/// Hold information about an object's styling
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub struct Style {
    foreground: Option<Color>,
    background: Option<Color>,
    italic: Option<bool>,
    bold: Option<bool>,
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string = String::new();
        if let Some(foreground) = self.foreground {
            string.push_str(&foreground.as_foreground())
        }
        if let Some(background) = self.background {
            string.push_str(&background.as_background());
        }

        // If it isn't specified, it's inherited from whatever crap was printed before it
        if let Some(italic) = self.italic {
            string.push_str(if italic { "\x1b[3m" } else { "\x1b[23m" });
        }
        if let Some(bold) = self.bold {
            string.push_str(if bold { "\x1b[1m" } else { "\x1b[22m" });
        }

        write!(f, "{string}")
    }
}

impl Style {
    /// Copy self, set foreground color, and return self
    pub fn foreground(mut self, color: Option<Color>) -> Self {
        self.foreground = color;
        self
    }

    /// Copy self, set background color, and return self
    pub fn background(mut self, color: Option<Color>) -> Self {
        self.background = color;
        self
    }

    /// Copy self, set whether to be italic, not be italic, or inherit, and return self
    pub fn italic(mut self, italic: Option<bool>) -> Self {
        self.italic = italic;
        self
    }

    /// Copy self, wet whether to be bold, not be bold, or inherit, and return self
    pub fn bold(mut self, bold: Option<bool>) -> Self {
        self.bold = bold;
        self
    }
}

/// # Color
/// Represents possible color values
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Color {
    RGB(u8, u8, u8),
    Red,
    Green,
    Blue,
    Black,
    White,
}

impl Color {
    /// Get the ANSI foreground code of this `Color`
    pub fn as_foreground(&self) -> String {
        match self {
            Self::Red => "31".to_string(),
            Self::Blue => "34".to_string(),
            Self::Green => "32".to_string(),
            Self::White => "37".to_string(),
            Self::Black => "30".to_string(),
            Self::RGB(r, g, b) => format!("\x1b[38;2;{r};{g};{b}m"),
        }
    }

    /// Get the ANSI background code of this `Color`
    pub fn as_background(&self) -> String {
        match self {
            Self::Red => "41".to_string(),
            Self::Blue => "44".to_string(),
            Self::Green => "42".to_string(),
            Self::White => "47".to_string(),
            Self::Black => "40".to_string(),
            Self::RGB(r, g, b) => format!("\x1b[48;2;{r};{g};{b}m"),
        }
    }
}
