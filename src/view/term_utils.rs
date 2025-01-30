use crate::core::Vec2D;
use std::{fmt, sync::OnceLock};
use terminal_size::terminal_size;

static TERMINAL_PREPARED: OnceLock<bool> = OnceLock::new();

/// Returns the size of the terminal as a `Vec2D`, using [`terminal_size::terminal_size()`]
#[must_use]
pub fn get_terminal_size_as_vec2d() -> Option<Vec2D> {
    let (width, height) = terminal_size()?;
    Some(Vec2D::new(i64::from(width.0), i64::from(height.0)))
}

/// Blocks the process until the console window is resized to fit `view_size`
pub fn block_until_resized(view_size: Vec2D) {
    if let Some(term_size) = get_terminal_size_as_vec2d() {
        if term_size.cmplt(view_size).any() {
            println!("Please resize your console window to fit the render\r");
            loop {
                let term_size = get_terminal_size_as_vec2d().unwrap_or_else(|| unreachable!());
                if term_size.cmpge(view_size).all() {
                    break;
                }
            }
        }
    }
}

/// Prepares the console by printing lines to move previous console lines out of the way. This is only done the first time this function is called, after which it does nothing
///
/// Returns an error if [`terminal_size`] returns `None`, or if it fails to write to the formatter
pub fn prepare_terminal(f: &mut fmt::Formatter<'_>) -> Result<(), String> {
    // If the console hasn't been prepared before
    if TERMINAL_PREPARED.get().is_none() {
        // Prevent the console from being prepared again
        TERMINAL_PREPARED.get_or_init(|| true);

        let Some((_, height)) = terminal_size() else {
            return Err(String::from("Couldn't get terminal size"));
        };

        write!(f, "{}", "\n".repeat(height.0 as usize)).map_err(|e| e.to_string())?;
    }

    Ok(())
}
