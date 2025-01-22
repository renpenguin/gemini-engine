use crate::core::{CanDraw, ColChar, Modifier, Vec2D};

use super::TextAlign;

/// Displays text at the given position
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Text {
    /// The position that the text is drawn from.
    pub pos: Vec2D,
    /// The actual text content of the element
    pub content: String,
    /// How the content should align to the `pos` property
    pub align: TextAlign,
    /// A raw [`Modifier`], determining the appearance of the `Text`
    pub modifier: Modifier,
}

impl Text {
    /// Create a new Text element with a position, content and modifier
    ///
    /// # Panics
    /// This function will panic if the content contains a newline, as Text only works with single lines. For multi-line strings, see [Sprite](super::Sprite)
    #[must_use]
    pub fn new(pos: Vec2D, content: &str, modifier: Modifier) -> Self {
        assert!(
            !content.contains('\n'),
            "Text was created with a content string containing a \n character"
        );

        Self {
            pos,
            content: String::from(content),
            align: TextAlign::Begin,
            modifier,
        }
    }

    /// Return the `Text` with an updated `align` property. Consumes the original `Text`
    #[must_use]
    pub const fn with_align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }
}

impl CanDraw for Text {
    fn draw_to(&self, canvas: &mut impl crate::core::Canvas) {
        let mut pos = self.pos;
        pos.x = self.align.apply_to(pos.x, self.content.len() as i64);

        for (x, text_char) in (0..).zip(self.content.chars()) {
            if text_char != ' ' {
                canvas.plot(
                    pos + Vec2D::new(x, 0),
                    ColChar::new(text_char, self.modifier),
                );
            }
        }
    }
}
