use crate::core::{CanDraw, ColChar, Vec2D};

/// The `Rect` takes a position and size, and returns a rectangle at that position with the given width and size when blit to a [`View`](super::super::View)
pub struct Rect {
    /// The position of the top-left corner of the `Rect`
    pub pos: Vec2D,
    /// The size of the `Rect`, extending from [`Rect::pos`]
    pub size: Vec2D,
    /// The [`ColChar`] used to fill the rectangle
    pub fill_char: ColChar,
}

impl Rect {
    /// Create a new rectangle using a given position, size and [`ColChar`]
    #[must_use]
    pub const fn new(pos: Vec2D, size: Vec2D, fill_char: ColChar) -> Self {
        Self {
            pos,
            size,
            fill_char,
        }
    }

    /// Create a new rectangle between two position to fill with a [`ColChar`]
    #[must_use]
    pub fn new_from_to(pos0: Vec2D, pos1: Vec2D, fill_char: ColChar) -> Self {
        Self::new(pos0, pos1 - pos0, fill_char)
    }
}

impl CanDraw for Rect {
    fn draw_to(&self, canvas: &mut impl crate::core::Canvas) {
        for x in 0..self.size.x {
            for y in 0..self.size.y {
                canvas.plot(self.pos + Vec2D::new(x, y), self.fill_char);
            }
        }
    }
}
