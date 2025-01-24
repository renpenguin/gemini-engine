use super::{Sprite, TextAlign2D};
use crate::core::{CanDraw, Modifier, Vec2D};

/// The `AnimatedSprite` struct contains a list of `String`s into which it indexes based on its `current_frame` property. You can cycle through frames with the [`AnimatedSprite::next_frame()`](AnimatedSprite::next_frame()) function
pub struct AnimatedSprite {
    /// The position from which the animated sprite will be drawn from
    pub pos: Vec2D,
    /// A collection of frames - ACII textures to be displayed by the `AnimatedSprite`
    pub frames: Vec<String>,
    /// The current frame being displayed. This will index directly into [`frames`](AnimatedSprite::frames)
    current_frame: usize,
    /// A raw [`Modifier`], determining the appearance of the `AnimatedSprite`
    pub modifier: Modifier,
    /// How the Sprite should align to the position
    pub align: TextAlign2D,
}

impl AnimatedSprite {
    /// Create a new `AnimatedSprite` struct. All newlines at the beginning of each texture will be removed
    #[must_use]
    pub fn new(pos: Vec2D, frames: &[&str], modifier: Modifier) -> Self {
        let processed_frames: Vec<String> = frames
            .iter()
            .map(|frame| frame.trim_start_matches('\n').into())
            .collect();

        Self {
            pos,
            frames: processed_frames,
            current_frame: 0,
            modifier,
            align: TextAlign2D::default(),
        }
    }

    /// Returns the current frame
    #[must_use]
    pub const fn get_current_frame(&self) -> usize {
        self.current_frame
    }

    /// Sets the current frame
    pub fn set_current_frame(&mut self, value: usize) {
        self.current_frame = value;
        self.current_frame = self.current_frame.rem_euclid(self.frames.len());
    }

    /// Go to the next frame of the `AnimatedSprite`'s frames. Will automatically wrap around at the end of the list
    pub fn next_frame(&mut self) {
        self.current_frame += 1;
        if self.current_frame >= self.frames.len() {
            self.current_frame = 0;
        }
    }
}

impl CanDraw for AnimatedSprite {
    fn draw_to(&self, canvas: &mut impl crate::core::Canvas) {
        Sprite::new(self.pos, &self.frames[self.current_frame], self.modifier)
            .with_align(self.align)
            .draw_to(canvas);
    }
}
