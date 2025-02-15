use crate::core::Vec2D;

/// An enum to determine the alignment of an [ascii](super) element's content
#[derive(Debug, Clone, Copy)]
pub enum TextAlign {
    /// Align to the beginning of the text
    Begin,
    /// Align to the center of the text
    Centered,
    /// Align to the end of the text
    End,
}

impl TextAlign {
    /// Align the given 1-dimentional coordinate as dictated by the `TextAlign` enum variation
    #[must_use]
    pub const fn apply_to(&self, pos: i64, text_length: i64) -> i64 {
        match self {
            Self::Begin => pos,
            Self::Centered => pos - text_length / 2,
            Self::End => pos - text_length,
        }
    }
}

/// Two-dimensional text align, used by [`Sprite`](super::Sprite) and [`AnimatedSprite`](super::AnimatedSprite)
#[derive(Debug, Clone, Copy)]
pub struct TextAlign2D {
    /// X coordinate [`TextAlign`]. `TextAlign::Begin` is left
    x: TextAlign,
    /// Y coordinate [`TextAlign`]. `TextAlign::Begin` is top
    y: TextAlign,
}

impl Default for TextAlign2D {
    fn default() -> Self {
        Self::new(TextAlign::Begin, TextAlign::Begin)
    }
}

impl TextAlign2D {
    /// Align to centre of text in both X and Y axes
    pub const CENTERED: Self = Self::new(TextAlign::Centered, TextAlign::Centered);

    /// Create a new `TextAlign2D` with a given x and y align
    #[must_use]
    pub const fn new(x_align: TextAlign, y_align: TextAlign) -> Self {
        Self {
            x: x_align,
            y: y_align,
        }
    }

    /// Align the given position as dictated by the X and Y `TextAlign` enum variations
    #[must_use]
    pub const fn apply_to(&self, pos: Vec2D, text_block_size: Vec2D) -> Vec2D {
        Vec2D::new(
            self.x.apply_to(pos.x, text_block_size.x),
            self.y.apply_to(pos.y, text_block_size.y),
        )
    }
}
