use super::Vec2D;

/// The wrapping mode is used to determine how you want to handle out-of-bounds pixels during plotting pixels to the screen. Here's how each possible value functions:
#[derive(Debug, Clone, Copy)]
pub enum WrappingMode {
    /// `WrappingMode::Wrap` wraps any out of bounds pixels around to the other side. This is useful if you have an object that travels the entirety of the screen and appears on the other side when it reaches the end.
    Wrap,
    /// `WrappingMode::Ignore` simply skips all out-of-bounds pixels. This is useful if you might have an object clipping through the edge of the screen but don't want it to wrap to the other side like [`WrappingMode::Wrap`] or panic and end the process like [`WrappingMode::Panic`]
    Ignore,
    /// `WrappingMode::Panic` will `panic!` if any pixels are out of bounds. You should use this if you have your own wrapping system implemented
    Panic,
}

impl WrappingMode {
    /// Handle the position based on the given bounds and the `WrappingMode` variation (See the [`WrappingMode`] documentation for more info)
    ///
    /// # Panics
    /// `WrappingMode::Panic` will panic if the position is out of bounds
    #[must_use]
    pub fn handle_bounds(&self, pos: Vec2D, bounds: Vec2D) -> Option<Vec2D> {
        let in_bounds_pos = pos.rem_euclid(bounds);

        match self {
            Self::Wrap => Some(in_bounds_pos),
            Self::Ignore => {
                if pos == in_bounds_pos {
                    Some(pos)
                } else {
                    None
                }
            }
            Self::Panic => {
                if pos == in_bounds_pos {
                    Some(pos)
                } else {
                    panic!("{pos} is out of bounds");
                }
            }
        }
    }
}
