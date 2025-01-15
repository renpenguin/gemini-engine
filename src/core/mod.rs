//! This module contains the core items that define the structure of the gemini renderer

mod colchar;
pub use colchar::{ColChar, Colour, Modifier};

/// An alias to an [`I64Vec2`](glam::I64Vec2), a two-dimensional vector of `i64` values
pub type Vec2D = glam::I64Vec2;

/// A struct that can be drawn to by elements which implement [`CanDraw`]
///
/// The only structs that implement this in `gemini` are [`View`](crate::view::View) and [`ScaleFitView`](crate::view::ScaleFitView).
pub trait Canvas: Sized {
	/// Plot a [`ColChar`] to the `Canvas` at `pos`
	fn plot(&mut self, pos: Vec2D, c: ColChar);
}

/// A struct that can draw to a [`Canvas`]
pub trait CanDraw {
	/// Draw the element to a struct that implements [`CanDraw`]
	fn draw_to(&self, canvas: &mut impl Canvas);
}
