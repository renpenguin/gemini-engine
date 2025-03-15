//! This module is home to the [`View`] struct, a [`Canvas`] that is able to draw to `stdout`.
use crate::core::{CanDraw, Canvas, ColChar, Vec2D};
use std::{
    fmt::{self, Display, Formatter},
    io::{self, Write},
};

mod scale_to_fit;
mod term_utils;
mod wrapping;

pub use scale_to_fit::ScaleFitView;
pub use wrapping::WrappingMode;

/// The View struct implements [`Canvas`], and can be used to draw to stdout. In normal use, you would clear the View, draw all your `CanDraws` implementing elements to it and then render to stdout with [`View::display_render`]. The following example demonstrates a piece of code that will render a View of width 9 and height 3, with a single Pixel in the middle
/// ```no_run
/// use gemini_engine::{view::{WrappingMode, View}, core::{ColChar, Vec2D}, primitives::Pixel};
///
/// let mut view = View::new(9, 3, ColChar::BACKGROUND)
///     .with_wrapping_mode(WrappingMode::Panic);
/// let pixel = Pixel::new(view.center(), ColChar::SOLID);
///
/// view.draw(&pixel);
///
/// view.display_render().unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct View {
    /// The width of the `View`. If modified, the View should be cleared to account for the new size
    pub width: usize,
    /// The height of the `View`. If modified, the View should be cleared to account for the new size
    pub height: usize,
    /// The character that the `View` will be filled with by default when [`View::clear`] is called
    pub background_char: ColChar,
    /// Determine how to handle pixels that are plotted outside the `View`
    pub wrapping_mode: WrappingMode,
    /// If true, [`View::display_render`] will block until the console window is resized to fit the `View`
    pub block_until_resized: bool,
    pixels: Vec<ColChar>,
}

impl View {
    /// Create a new `View`
    #[must_use]
    pub fn new(width: usize, height: usize, background_char: ColChar) -> Self {
        let mut view = Self {
            width,
            height,
            background_char,
            wrapping_mode: WrappingMode::Ignore,
            block_until_resized: false,
            pixels: Vec::with_capacity(width * height),
        };
        view.clear();

        view
    }

    /// Return the `View` with an updated `wrapping_mode` property. Consumes the original `View`
    ///
    /// ## Example
    /// ```
    /// # use gemini_engine::{view::{View, WrappingMode}, core::{ColChar, Vec2D, Canvas}};
    /// let mut view = View::new(20, 7, ColChar::BACKGROUND)
    ///     .with_wrapping_mode(WrappingMode::Wrap);
    /// // The pixel will be wrapped and drawn at `(0, 4)`
    /// view.plot(Vec2D::new(20,4), ColChar::SOLID);
    /// ```
    #[must_use]
    pub const fn with_wrapping_mode(mut self, wrapping_mode: WrappingMode) -> Self {
        self.wrapping_mode = wrapping_mode;
        self
    }

    /// Return the `View` with an updated `block_until_resized` property. Consumes the original `View`
    ///
    /// ## Example
    /// ```no_run
    /// # use gemini_engine::{view::{View, WrappingMode}, core::ColChar};
    /// let mut view = View::new(20, 7, ColChar::BACKGROUND)
    ///     .with_block_until_resized();
    /// // If the terminal size is smaller than (20, 7), this will wait until the terminal has been resized
    /// view.display_render().unwrap();
    /// ```
    #[must_use]
    pub const fn with_block_until_resized(mut self) -> Self {
        self.block_until_resized = true;
        self
    }

    /// Return the width and height of the `View` as a [`Vec2D`]
    #[must_use]
    pub const fn size(&self) -> Vec2D {
        Vec2D::new(self.width as i64, self.height as i64)
    }

    /// Return [`Vec2D`] coordinates of the centre of the `View`
    #[must_use]
    pub fn center(&self) -> Vec2D {
        self.size() / 2
    }

    /// Clear the `View` of all pixels, overwriting them all with the set `background_char`
    pub fn clear(&mut self) {
        self.pixels = vec![self.background_char; self.width * self.height];
    }

    /// Draw a struct implementing [`CanDraw`] to the `View`
    #[inline]
    pub fn draw(&mut self, element: &impl CanDraw) {
        element.draw_to(self);
    }

    /// Draw a struct implementing [`CanDraw`] to the `View` with a doubled width. Drawing a `Pixel` at `Vec2D(5,3)`, for example, will result in pixels at at `Vec2D(10,3)` and `Vec2D(11,3)` being plotted to. Useful when you want to work with more square pixels, as single text characters are much taller than they are wide
    pub fn draw_double_width(&mut self, element: &impl CanDraw) {
        struct DoubleWidthView<'v>(&'v mut View);
        impl Canvas for DoubleWidthView<'_> {
            fn plot(&mut self, pos: Vec2D, c: ColChar) {
                let pos = pos * Vec2D::new(2, 1);
                self.0.plot(pos, c);
                self.0.plot(pos + Vec2D::new(1, 0), c);
            }
        }

        // Wrap the `View` in a custom struct (defined above), replacing the plot function with one that plots at double width, and pass it to the element as usual. This should be much faster and more memory efficient than storing all of the element's draw calls in a `PixelContainer` before double-width plotting each of them.
        element.draw_to(&mut DoubleWidthView(self));
    }

    /// Display the `View`. `View` implements the `Display` trait and so can be rendered in many ways (such as `println!("{view}");`), but this is intended to be the fastest way possible.
    ///
    /// # Errors
    /// Returns the `Result` from writing to `io::stdout().lock()`. You can simply ignore it with `let _ =` or `.unwrap()` most of the time
    pub fn display_render(&self) -> io::Result<()> {
        let mut stdout = io::stdout().lock();
        if self.block_until_resized {
            let view_size = self.size();
            term_utils::block_until_resized(view_size);
        }

        write!(stdout, "{self}")
    }
}

impl Canvas for View {
    /// Plot a pixel to the `View`. Accepts a [`Vec2D`] (the position of the pixel) and a [`ColChar`] (what the pixel should look like/what colour it should be)
    ///
    /// # Panics
    /// Will panic if the position is out of bounds of the `View` and `wrapping_mode` is `WrappingMode::Panic`
    fn plot(&mut self, pos: Vec2D, c: ColChar) {
        if let Some(wrapped_pos) = self.wrapping_mode.handle_bounds(pos, self.size()) {
            let i = self.width * wrapped_pos.y.unsigned_abs() as usize
                + wrapped_pos.x.unsigned_abs() as usize;
            self.pixels[i] = c;
        }
    }
}

impl Display for View {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        term_utils::prepare_terminal(f).map_err(|_| fmt::Error)?;

        f.write_str("\x1b[H\x1b[J")?;
        for y in 0..self.height {
            let row = &self.pixels[self.width * y..self.width * (y + 1)];

            for x in 0..row.len() {
                row[x].display_with_prev_and_next(
                    f,
                    row.get(x - 1).map(|c| c.modifier),
                    row.get(x + 1).map(|c| c.modifier),
                )?;
            }
            f.write_str("\r\n")?;
        }
        f.write_str("\x1b[J")?;

        Ok(())
    }
}
