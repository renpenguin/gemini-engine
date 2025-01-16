use super::Colour;
use std::fmt::Display;

/// The [`Modifier`] enum is used for adding modifications to text such as colour, bold/italic/underline and others. `Modifier` should be used through [`ColChar`](super::ColChar).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Modifier {
    /// `Coded(u8)` unwraps to `\x1b[{x}m`, where `x` is the code.
    ///
    /// For example, `Coded(0)` (available as `Modifier::END`) clears all previously applied modifiers. When displayed, `Modifier::Coded(31)` writes `\x1b[31m`.
    ///
    /// See <https://prirai.github.io/blogs/ansi-esc/#colors-graphics-mode> for a guide to available code
    Coded(u8),
    /// `Colour(`[`Colour`](Colour)`)` unwraps to `\x1b[38;2;{r};{g};{b}m`, where `(r, g, b)` together represent a 24 bit RGB value
    ///
    /// Not all terminals support RGB ANSI escape codes, in which case you will have to resort to `Modifier::Coded` for colours. Some `Coded` colours are available as constants, e.g. [`Modifier::RED`]
    Colour(Colour),
    /// `None` unwraps to nothing. It does not change the current applied modifiers.
    #[default]
    None,
}

impl Modifier {
    /// An END code, which clears all previously applied modifiers. You should never have to use this yourself as `View` makes use of it between pixels where necessary
    pub const END: Self = Self::Coded(0);
    /// A red ANSI escape code
    pub const RED: Self = Self::Coded(31);
    /// A green ANSI escape code
    pub const GREEN: Self = Self::Coded(32);
    /// A yellow ANSI escape code
    pub const YELLOW: Self = Self::Coded(33);
    /// A blue ANSI escape code
    pub const BLUE: Self = Self::Coded(34);
    /// A purple ANSI escape code
    pub const PURPLE: Self = Self::Coded(35);
    /// A cyan ANSI escape code
    pub const CYAN: Self = Self::Coded(36);

    /// Create a `Modifier::Colour` from an RGB value
    #[must_use]
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::Colour(Colour::rgb(r, g, b))
    }

    /// Create a `Modifier::Colour` from an HSV value
    #[must_use]
    pub fn from_hsv(h: u8, s: u8, v: u8) -> Self {
        Self::Colour(Colour::hsv(h, s, v))
    }
}

impl Display for Modifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Coded(code) => write!(f, "\x1b[{code}m"),
            Self::Colour(c) => write!(f, "\x1b[38;2;{};{};{}m", c.r, c.g, c.b),
            Self::None => Ok(()),
        }
    }
}
