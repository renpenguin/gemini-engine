use super::{Text, TextAlign2D};
use crate::core::{CanDraw, Modifier, Vec2D};

/// The `Sprite` takes a multi-line string as a parameter, and can be used to draw ASCII art to a `Canvas`
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Sprite {
    /// The position from which the sprite will be drawn from
    pub pos: Vec2D,
    /// The ACII texture (pun intended) displayed by the `Sprite`
    pub texture: String,
    /// A raw [`Modifier`], determining the appearance of the `Sprite`
    pub modifier: Modifier,
    /// How the Sprite should align to the position
    pub align: TextAlign2D,
}

impl Sprite {
    /// Create a new `Sprite` struct. All newlines at the beginning of the texture will be removed
    #[must_use]
    pub fn new(pos: Vec2D, texture: &str, modifier: Modifier) -> Self {
        Self {
            pos,
            texture: texture.trim_start_matches('\n').into(),
            modifier,
            align: TextAlign2D::default(),
        }
    }

    /// Return the `Sprite` with an updated `align` property. Consumes the original `Sprite`
    #[must_use]
    pub const fn with_align(mut self, align: TextAlign2D) -> Self {
        self.align = align;
        self
    }
}

impl CanDraw for Sprite {
    fn draw_to(&self, canvas: &mut impl crate::core::Canvas) {
        let content_size = Vec2D::new(
            self.texture.lines().map(str::len).max().unwrap_or(0) as i64,
            self.texture.lines().count() as i64,
        );
        let pos = self.align.apply_to(self.pos, content_size);

        let lines = self.texture.split('\n');
        for (y, line) in (0..).zip(lines) {
            Text::new(pos + Vec2D::new(0, y), line, self.modifier).draw_to(canvas);
        }
    }
}
