use crate::core::{CanDraw, ColChar, Vec2D};

/// A rectangle primitive which implements [`CanDraw`], and so can be drawn to [Canvas](crate::core::Canvas)es
pub struct Rect {
    /// The position of the top-left corner of the `Rect`
    pub pos: Vec2D,
    /// The size of the `Rect`, extending from `pos`
    pub size: Vec2D,
    /// The [`ColChar`] used to fill the rectangle
    pub fill_char: ColChar,
}

impl Rect {
    /// Create a new `Rect` using a position and size
    #[must_use]
    pub const fn new(pos: Vec2D, size: Vec2D, fill_char: ColChar) -> Self {
        Self {
            pos,
            size,
            fill_char,
        }
    }

    /// Create a new `Rect` using two positions
    #[must_use]
    pub fn new_from_to(top_left: Vec2D, bottom_right: Vec2D, fill_char: ColChar) -> Self {
        Self::new(top_left, bottom_right - top_left + Vec2D::ONE, fill_char)
    }

    /// Return the coordinates of the bottom right point
    #[must_use]
    pub fn bottom_right(&self) -> Vec2D {
        self.pos + self.size - Vec2D::ONE
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
