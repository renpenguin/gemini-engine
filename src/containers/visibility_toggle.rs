use crate::core::CanDraw;

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

impl<T: CanDraw> CanDraw for VisibilityToggle<T> {
    fn draw_to(&self, canvas: &mut impl crate::core::Canvas) {
        if self.visible {
            self.element.draw_to(canvas);
        }
    }
}
