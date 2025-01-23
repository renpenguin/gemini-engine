use crate::core::{CanDraw, ColChar, Vec2D};

/// A singular point with a [`Vec2D`] position and [`ColChar`]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pixel {
    /// The position of the `Pixel`
    pub pos: Vec2D,
    /// The appearance/colour of the `Pixel`
    pub fill_char: ColChar,
}

impl Pixel {
    /// Create a new `Pixel`
    #[must_use]
    pub const fn new(pos: Vec2D, fill_char: ColChar) -> Self {
        Self { pos, fill_char }
    }
}

impl CanDraw for Pixel {
    fn draw_to(&self, canvas: &mut impl crate::core::Canvas) {
        canvas.plot(self.pos, self.fill_char);
    }
}
