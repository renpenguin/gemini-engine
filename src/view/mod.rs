//! This module is home to the [`View`] struct, which handles the printing of pixels to an ANSI standard text output
use crate::{
    containers::PixelContainer,
    core::{CanDraw, Canvas, ColChar, Vec2D},
};
use std::{
    fmt::{self, Display, Formatter},
    io::{self, Write},
};

mod scale_to_fit;
mod term_utils;
mod wrapping;

pub use scale_to_fit::ScaleFitView;
pub use wrapping::WrappingMode;

/// The View struct is the canvas on which you will print all of your `ViewElement`s. In normal use, you would clear the View, `blit` all your `ViewElement`s to it and then render. The following example demonstrates a piece of code that will render a View of width 9 and height 3, with a single Pixel in the middle
/// ```
/// use gemini_engine::elements::{view::{Wrapping, ColChar}, View, Pixel, Vec2D};
///
/// let mut view = View::new(9, 3, ColChar::BACKGROUND)
///     .with_wrapping_mode(Wrapping::Panic);
/// let pixel = Pixel::new(view.center(), ColChar::SOLID);
///
/// view.draw(&pixel);
///
/// view.display_render().unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct View {
    /// The width of the `View`
    pub width: usize,
    /// The height of the `View`
    pub height: usize,
    /// The character that the `View` will be filled with by default on clear
    pub background_char: ColChar,
    /// Determine how to handle pixels that are plotted outside the `View`
    pub wrapping_mode: WrappingMode,
    /// If true, [`View.display_render`] will block until the console window is resized to fit the `View`
    pub block_until_resized: bool,
    pixels: Vec<ColChar>,
}

impl View {
    /// Create a new `View` using [`width`](View::width), [`height`](View::height) and [`background_char`](View::background_char) parameters
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

    /// Return the `View` with its [`wrapping_mode`](View::wrapping_mode) field set to the chosen value. Consumes the original `View`
    #[must_use]
    pub const fn with_wrapping_mode(mut self, wrapping_mode: WrappingMode) -> Self {
        self.wrapping_mode = wrapping_mode;
        self
    }

    /// Return the `View` with its [`block_until_resized`](View::block_until_resized) field set to the chosen value. Consumes the original `View`
    #[must_use]
    pub const fn with_block_until_resized(mut self, block_until_resized: bool) -> Self {
        self.block_until_resized = block_until_resized;
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

    /// Clear the `View` of all pixels
    pub fn clear(&mut self) {
        self.pixels = vec![self.background_char; self.width * self.height];
    }

    /// Draw a struct implementing [`CanDraw`] to the `Canvas`
    #[inline]
    pub fn draw(&mut self, element: &impl CanDraw) {
        element.draw_to(self);
    }

    /// Blit a struct implementing [`CanDraw`] to the `View` with a doubled width. Drawing a `Pixel` at `Vec2D(5,3)`, for example, will result in pixels at at `Vec2D(10,3)` and `Vec2D(11,3)` being plotted to. Useful when you want to work with more square pixels, as single text characters are much taller than they are wide
    pub fn draw_double_width(&mut self, element: &impl CanDraw) {
        for mut pixel in PixelContainer::from(element).pixels {
            pixel.pos.x *= 2;
            self.draw(&pixel);
            pixel.pos.x += 1;
            self.draw(&pixel);
        }
    }

    /// Display the `View`. `View` implements the `Display` trait and so can be rendered in many ways (such as `println!("{view}");`), but this is intended to be the fastest way possible.
    ///
    /// # Errors
    /// Returns the `Result` from writing to `io::stdout().lock()`. You can ignore it with `let _ = ...` most of the time
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
    /// Plot a pixel to the `View`. Accepts a [`Vec2D`] (the position of the pixel), [`ColChar`] (what the pixel should look like/what colour it should be), and a [`WrappingMode`] enum variant (Please see the [`WrappingMode`] documentation for more info)
    fn plot(&mut self, pos: Vec2D, c: ColChar) {
        if let Some(wrapped_pos) = self.wrapping_mode.handle_bounds(pos, self.size()) {
            let i = self.width * wrapped_pos.y.unsigned_abs() as usize + wrapped_pos.x.unsigned_abs() as usize;
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

            row[0].display_with_prev_and_next(f, None, Some(row[1].modifier))?;
            for x in 1..(row.len() - 1) {
                row[x].display_with_prev_and_next(
                    f,
                    Some(row[x - 1].modifier),
                    Some(row[x + 1].modifier),
                )?;
            }
            row[row.len() - 1].display_with_prev_and_next(
                f,
                Some(row[row.len() - 2].modifier),
                None,
            )?;
            f.write_str("\r\n")?;
        }
        f.write_str("\x1b[J")?;

        Ok(())
    }
}
