use crate::core::{ColChar, Vec2D, CanDraw};

use super::Line;

/// A triangle primitive which implements [`CanDraw`], and so can be drawn to [Canvas](crate::core::Canvas)es
pub struct Triangle {
    /// The 3 corners of the triangle
    pub corners: [Vec2D; 3],
    /// The [`ColChar`] used to fill the triangle
    pub fill_char: ColChar,
}

impl Triangle {
    /// Create a new `Triangle` from three separate positions and a `ColChar`
    #[must_use]
    pub const fn new(pos0: Vec2D, pos1: Vec2D, pos2: Vec2D, fill_char: ColChar) -> Self {
        Self::with_array([pos0, pos1, pos2], fill_char)
    }

    /// Create a new `Triangle` from an array of `Vec2D`s and a `ColChar`
    #[must_use]
    pub const fn with_array(corners: [Vec2D; 3], fill_char: ColChar) -> Self {
        Self { corners, fill_char }
    }
}

impl CanDraw for Triangle {
    fn draw_to(&self, canvas: &mut impl crate::core::Canvas) {
        let mut corners = self.corners;
        corners.sort_unstable_by_key(|k| k.y);
        let (x0, y0) = corners[0].into();
        let (x1, y1) = corners[1].into();
        let (x2, y2) = corners[2].into();

        let mut x01 = super::interpolate(y0, x0, y1, x1);
        let x12 = super::interpolate(y1, x1, y2, x2);
        let x02 = super::interpolate(y0, x0, y2, x2);

        // Concat the two shorter sides
        x01.pop();
        let x01_12 = [x01, x12].concat();

        let m = (x01_12.len() as f64 / 2.0).floor() as usize;
        let (x_left, x_right) = if x02[m] < x01_12[m] {
            (x02, x01_12)
        } else {
            (x01_12, x02)
        };

        for (i, y) in (y0..y2).enumerate() {
            for x in x_left[i]..x_right[i] {
                canvas.plot(Vec2D::new(x, y), self.fill_char);
            }
        }

        // Outline (will probably remove later)
        Line::new(corners[0], corners[1], self.fill_char).draw_to(canvas);
        Line::new(corners[1], corners[2], self.fill_char).draw_to(canvas);
        Line::new(corners[2], corners[0], self.fill_char).draw_to(canvas);
    }
}
