use super::Triangle;
use crate::core::{CanDraw, ColChar, Vec2D};

/// A `Polygon` draws a polygon with the chosen vertices by triangulating them
pub struct Polygon {
    /// The vertices that make up the `Polygon`
    pub vertices: Vec<Vec2D>,
    /// The [`ColChar`] used to fill the `Polygon`
    pub fill_char: ColChar,
}

impl Polygon {
    /// Create a new `Polygon`
    #[must_use]
    pub fn new(vertices: &[Vec2D], fill_char: ColChar) -> Self {
        Self {
            vertices: vertices.to_vec(),
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
