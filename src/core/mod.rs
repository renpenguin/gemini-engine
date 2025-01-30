//! [`core`] declares the relationship between any [`Canvas`](Canvas) (an object that can be drawn to) and the library's primitives and anything else that can be drawn to the screen.
//!
//! ## Quick Start
//! Let's get started with a simple program to demonstrate how Gemini works:
//! ```no_run
#![doc = include_str!("../../examples/quick-start.rs")]
//! ```
//! Ok, let's go over this and see what's going on. We start by creating a [`View`](crate::view::View) and [`Pixel`](crate::primitives::Pixel). `View::new` takes a width and height parameter, as well as a [`ColChar`](crate::core::ColChar) which is used as the default character when the screen is cleared. We also set the [`WrappingMode`](crate::view::WrappingMode) to `Wrap`, which means that any pixel drawn outside the screen will be wrapped back around to the opposite edge.
//!
//! We use [`ColChar`](crate::core::ColChar) to define the text character and colour used to represent each pixel. Here we used the `ColChar::BACKGROUND` and `ColChar::SOLID` constants, which appear as `░` and `█` respectively.
//!
//! `Vec2D` is an alias to [`glam::I64Vec2`]. We used it here to define the `Pixel`'s starting position, before the game loop.
//!
//! Now that we've got initialisation out of the way, let's get on to the main loop. In Gemini the main loop generally goes as follows:
//! 1. Logical processing (physics, input, etc.)
//! 2. Clear the [`View`](crate::view::View)
//! 3. Draw all the [`CanDraw`] elements to the `View`
//! 4. Call [`View.display_render`](crate::view::View::display_render)
//! 5. Wait until the next frame
//!
//! In our case, we want to move our `Pixel` one unit to the right every frame, so we do so with `pixel.pos.x += 1;`. Next we draw the `Pixel` to the `View` and call `display_render()`, which prints the render to `stdout` (make sure your terminal is large enough to fit the whole image!). The last line of our code sleeps for `1/FPS` seconds. We pass `None` to the elapsed parameter here, but we could instead pass the amount of time taken for our gameloop to run (as a [`Duration`](std::time::Duration)), which would be subtracted from the total sleep time.
//!
//! There you have it! You've written your first program with Gemini! This is still a work in progress, so any feedback or issue requests would be appreciated :)

mod colchar;
pub use colchar::{ColChar, Colour, Modifier};

/// An alias to [`I64Vec2`](glam::I64Vec2), a two-dimensional vector of `i64` values
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
