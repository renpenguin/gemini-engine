use super::Triangle;
use crate::core::{CanDraw, ColChar, Vec2D};

/// The `Polygon` takes a vec of [`Vec2D`]s and returns a polygon with those vertices when blit to a [`View`](super::super::View)
pub struct Polygon {
    /// The vertices that make up the polygon
    pub vertices: Vec<Vec2D>,
    /// The [`ColChar`] used to fill the polygon
    pub fill_char: ColChar,
}

impl Polygon {
    /// Create a new polygon
    #[must_use]
    pub const fn new(vertices: Vec<Vec2D>, fill_char: ColChar) -> Self {
        Self {
            vertices,
            fill_char,
        }
    }
}

impl CanDraw for Polygon {
    fn draw_to(&self, canvas: &mut impl crate::core::Canvas) {
        super::triangulate(&self.vertices)
            .into_iter()
            .map(|corners| Triangle::with_array(corners, self.fill_char))
            .for_each(|t| t.draw_to(canvas));
    }
}
