use crate::core::{ColChar, CanDraw, Vec2D};

/// A line primitive which implements [`CanDraw`], and so can be drawn to [Canvas](crate::core::Canvas)es
pub struct Line {
    /// The start positon of the line
    pub pos0: Vec2D,
    /// The end position of the line
    pub pos1: Vec2D,
    /// The [`ColChar`] used to colour the line
    pub fill_char: ColChar,
}

impl Line {
    /// Create a new `Line` with a start and end point and a [`ColChar`]
    #[must_use]
    pub const fn new(pos0: Vec2D, pos1: Vec2D, fill_char: ColChar) -> Self {
        Self {
            pos0,
            pos1,
            fill_char,
        }
    }
}

impl CanDraw for Line {
    fn draw_to(&self, canvas: &mut impl crate::core::Canvas) {
        let (mut x, mut y) = self.pos0.into();
        let (x1, y1) = self.pos1.into();

        let dx = (x1 - x).abs();
        let sx = if x < x1 { 1 } else { -1 };
        let dy = -(y1 - y).abs();
        let sy = if y < y1 { 1 } else { -1 };
        let mut error = dx + dy;

        loop {
            canvas.plot(Vec2D::new(x, y), self.fill_char);
            let e2 = error * 2;
            if e2 >= dy {
                if x == x1 {
                    break;
                };
                error += dy;
                x += sx;
            };
            if e2 <= dx {
                if y == y1 {
                    break;
                };
                error += dx;
                y += sy;
            };
        }
    }
}
