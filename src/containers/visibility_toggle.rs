use crate::core::CanDraw;

use super::CanCollide;

/// `VisibilityToggle` is a container for a [`CanDraw`] with a property `visible`. When drawn to a `Canvas` the contained element will only appear if `visible` is `true`
#[derive(Debug, Clone)]
pub struct VisibilityToggle<E: CanDraw> {
    /// The element held by the `VisibilityToggle`. Must implement [`CanDraw`]
    pub element: E,
    /// Whether the element is visible
    pub visible: bool,
}

impl<E: CanDraw> VisibilityToggle<E> {
    /// Creates a new `VisibilityToggle` with the visibility set to true
    pub const fn new(element: E) -> Self {
        Self {
            element,
            visible: true,
        }
    }
}

impl<E: CanDraw> CanDraw for VisibilityToggle<E> {
    fn draw_to(&self, canvas: &mut impl crate::core::Canvas) {
        if self.visible {
            self.element.draw_to(canvas);
        }
    }
}

impl<E: CanDraw + CanCollide> CanCollide for VisibilityToggle<E> {
    fn collides_with_pos(&self, pos: crate::core::Vec2D) -> bool {
        if self.visible {
            self.element.collides_with_pos(pos)
        } else {
            false
        }
    }
}
