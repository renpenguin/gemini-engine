use crate::{
    core::{CanDraw, Canvas, ColChar, Vec2D},
    primitives::Pixel,
};

use super::CanCollide;

/// A `PixelContainer` acts as an intermediary between [Canvas]es and structs implementing [`CanDraw`]. Objects can draw to the `PixelContainer`, which in turn can draw to any other `Canvas` implementing struct
#[derive(Debug, Clone)]
pub struct PixelContainer {
    /// Any pixels plotted to the `PixelContainer` are stored here
    pub pixels: Vec<Pixel>,
}

impl Default for PixelContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl PixelContainer {
    /// Create a new, empty `PixelContainer`
    #[must_use]
    pub const fn new() -> Self {
        Self { pixels: vec![] }
    }

    /// Plot a pixel to the `PixelContainer`
    pub fn plot(&mut self, pos: Vec2D, c: ColChar) {
        self.pixels.push(Pixel::new(pos, c));
    }

    /// Moves all the pixels of `other` into `self`, leaving `other` empty
    pub fn append(&mut self, pixels: &mut Vec<Pixel>) {
        self.pixels.append(pixels);
    }

    /// Append a slice of `Vec2D` points which all share a [`ColChar`]
    pub fn append_points(&mut self, points: &[Vec2D], fill_char: ColChar) {
        for point in points {
            self.plot(*point, fill_char);
        }
    }

    /// Draw a struct implementing [`CanDraw`] to the `PixelContainer`.
    pub fn draw(&mut self, element: &impl CanDraw) {
        element.draw_to(self);
    }
}

impl From<&[Pixel]> for PixelContainer {
    fn from(pixels: &[Pixel]) -> Self {
        Self {
            pixels: pixels.to_vec(),
        }
    }
}

impl<E: CanDraw> From<&E> for PixelContainer {
    /// Create a new `PixelContainer` with the pixels of the passed element
    fn from(element: &E) -> Self {
        let mut container = Self::new();
        container.draw(element);
        container
    }
}

// TODO: Do i need this?
// impl From<&[(Vec2D, ColChar)]> for PixelContainer {
//     fn from(pixels: &[(Vec2D, ColChar)]) -> Self {
//         Self {
//             pixels: pixels.iter().map(|x| Pixel::from(*x)).collect(),
//         }
//     }
// }

impl From<(&[Vec2D], ColChar)> for PixelContainer {
    fn from(value: (&[Vec2D], ColChar)) -> Self {
        Self {
            pixels: value
                .0
                .iter()
                .map(|pos| Pixel::new(*pos, value.1))
                .collect(),
        }
    }
}

impl Canvas for PixelContainer {
    fn plot(&mut self, pos: Vec2D, c: ColChar) {
        self.plot(pos, c);
    }
}

impl CanDraw for PixelContainer {
    fn draw_to(&self, canvas: &mut impl Canvas) {
        for pixel in &self.pixels {
            canvas.plot(pixel.pos, pixel.fill_char);
        }
    }
}

impl CanCollide for PixelContainer {
    fn collides_with_pos(&self, pos: Vec2D) -> bool {
        self.pixels.iter().any(|p| p.pos == pos)
    }
}
