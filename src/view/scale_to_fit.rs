use crate::{core::Canvas, view::term_utils};

use super::{ColChar, Vec2D, View};

/// A wrapper around a [`View`] which auto resizes to fit the terminal window
///
/// `ScaleFitView`'s [`update()`](ScaleFitView::update()) function should be used in place of `View`'s `clear()` function to handle auto-resizing and clearing
#[non_exhaustive]
pub struct ScaleFitView {
    /// The [`View`] that this struct wraps around
    pub view: View,
    /// How many rows to leave clear below the rendered view. You might want to set this if you have more than one line of text after rendered text
    pub empty_row_count: i64,
}

impl ScaleFitView {
    /// Create a new `ScaleFitView` with the given background `ColChar`
    #[must_use]
    pub fn new(background_char: ColChar) -> Self {
        let mut tmp = Self {
            view: View::new(0, 0, background_char),
            empty_row_count: 1,
        };
        tmp.update();
        tmp
    }

    /// Return the `ScaleFitView` with an updated `empty_row_count` property. Consumes the original `ScaleFitView`
    #[must_use]
    pub const fn with_empty_row_count(mut self, empty_row_count: i64) -> Self {
        self.empty_row_count = empty_row_count;
        self
    }

    /// Returns the size of the terminal, with the y adjusted as intended using the [`empty_row_count`](ScaleFitView::empty_row_count) property
    ///
    /// # Panics
    /// Panics if there is no TTY to get the terminal size of, as per [`terminal_size::terminal_size()`]
    #[must_use]
    pub fn intended_size(&self) -> Vec2D {
        let mut term_size =
            term_utils::get_terminal_size_as_vec2d().expect("Failed to get terminal size");
        term_size.y -= self.empty_row_count + 1;

        term_size.max(Vec2D::ZERO)
    }

    /// Resize and clear the `View`
    ///
    /// # Panics
    /// Panics if there is no TTY to get the terminal size of, as per [`terminal_size::terminal_size()`]
    pub fn update(&mut self) {
        let term_size = self.intended_size();
        self.view.width = term_size.x as usize;
        self.view.height = term_size.y as usize;

        self.view.clear();
    }
}

impl Canvas for ScaleFitView {
    fn plot(&mut self, pos: Vec2D, c: ColChar) {
        self.view.plot(pos, c);
    }
}
