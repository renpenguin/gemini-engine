use crate::core::{CanDraw, ColChar, Vec2D};

/// The `Pixel` holds a single [`Vec2D`] (the coordinates at which it is printed when blit to a [`View`](super::View)) and a [`ColChar`]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pixel {
    /// The position of the `Pixel`
    pub pos: Vec2D,
    /// The appearance/colour of the `Pixel`
    pub fill_char: ColChar,
}

impl Pixel {
    /// Create a new `Pixel` from a [`Vec2D`] and [`ColChar`]
    #[must_use]
    pub const fn new(pos: Vec2D, fill_char: ColChar) -> Self {
        Self { pos, fill_char }
    }
}

// TODO: figure out if we need this?
// impl From<(Vec2D, ColChar)> for Pixel {
//     fn from(value: (Vec2D, ColChar)) -> Self {
//         Self {
//             pos: value.0,
//             fill_char: value.1,
//         }
//     }
// }

impl CanDraw for Pixel {
    fn draw_to(&self, canvas: &mut impl crate::core::Canvas) {
        canvas.plot(self.pos, self.fill_char);
    }
}
