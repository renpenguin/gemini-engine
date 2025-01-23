use std::fmt::{self, Debug, Display};

mod colour;
mod modifier;

pub use colour::Colour;
pub use modifier::Modifier;

/// A coloured character. Made up of `text_char`, a single ascii character used as the "pixel" when drawn to a [`Canvas`](super::Canvas), and `modifier`, which gives that pixel a colour or makes it bold/italic
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColChar {
    /// A single ascii character used as the "pixel" when drawn to a [`Canvas`](super::Canvas)
    pub text_char: char,
    /// Defines the appearance of the character - colour, bold/italic, etc.
    pub modifier: Modifier,
}

impl ColChar {
    /// A solid █ character with no [`Modifier`].
    ///
    /// ## Example
    /// Using a sequence like this will create a red █ `ColChar`
    /// ```rs
    /// ColChar::SOLID.with_rgb(255, 0, 0)
    /// ```
    pub const SOLID: Self = Self {
        text_char: '█',
        modifier: Modifier::None,
    };
    /// A less solid ░ character with no [`Modifier`]
    pub const BACKGROUND: Self = Self {
        text_char: '░',
        modifier: Modifier::None,
    };
    /// A whitespace character with no [`Modifier`]
    pub const EMPTY: Self = Self {
        text_char: ' ',
        modifier: Modifier::None,
    };
    /// An opaque whitespace character (`\u{2008}`) with no [`Modifier`]
    ///
    /// ASCII Whitespaces are interpreted as transparent by [`ascii`](crate::ascii) elements. If you want opacity, use this void character
    pub const VOID: Self = Self {
        text_char: ' ', // \u{2008}
        modifier: Modifier::None,
    };

    /// Create a new `ColChar` with a text character and a [`Modifier`]
    #[must_use]
    pub const fn new(text_char: char, modifier: Modifier) -> Self {
        Self {
            text_char,
            modifier,
        }
    }

    /// Return a `ColChar` with the same modifier and new `text_char`
    #[must_use]
    pub const fn with_char(mut self, text_char: char) -> Self {
        self.text_char = text_char;
        self
    }

    /// Return a `ColChar` with the same `text_char` and new modifier
    #[must_use]
    pub const fn with_mod(mut self, modifier: Modifier) -> Self {
        self.modifier = modifier;
        self
    }

    /// Return a `ColChar` with the same `text_char` and new `Modifier::Colour` modifier
    #[must_use]
    pub const fn with_colour(mut self, colour: Colour) -> Self {
        self.modifier = Modifier::Colour(colour);
        self
    }

    /// Return a `ColChar` with the same `text_char` and new `Modifier::Colour` modifier from an RGB value
    #[must_use]
    pub const fn with_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.modifier = Modifier::from_rgb(r, g, b);
        self
    }

    /// Return a `ColChar` with the same `text_char` and new `Modifier::Colour` modifier from an HSV value
    #[must_use]
    pub fn with_hsv(mut self, h: u8, s: u8, v: u8) -> Self {
        self.modifier = Modifier::from_hsv(h, s, v);
        self
    }

    /// Return the displayed `ColChar`, omitting the `Modifier`s where necessary
    pub(crate) fn display_with_prev_and_next(
        self,
        f: &mut fmt::Formatter,
        prev_mod: Option<Modifier>,
        next_mod: Option<Modifier>,
    ) -> fmt::Result {
        let modifier = if prev_mod == Some(self.modifier) {
            Modifier::None
        } else {
            self.modifier
        };
        let end = if next_mod == Some(self.modifier) {
            Modifier::None
        } else {
            Modifier::END
        };

        write!(f, "{}{}{}", modifier, self.text_char, end)
    }
}

impl Default for ColChar {
    fn default() -> Self {
        Self::SOLID
    }
}

impl Display for ColChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.modifier {
            Modifier::None => write!(f, "{}", self.text_char),
            _ => write!(f, "{}{}{}", self.modifier, self.text_char, Modifier::END),
        }
    }
}
