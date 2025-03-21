use crate::{
    containers::PixelContainer,
    core::{CanDraw, Vec2D},
};

/// Must be implemented to be used by the [`CollisionContainer`]
pub trait CanCollide {
    /// Returns `true` if the collider intersects the passed position
    #[must_use]
    fn collides_with_pos(&self, pos: Vec2D) -> bool;
}

/// Container for references to collider objects
#[derive(Clone)]
pub struct CollisionContainer<'e> {
    /// The elements used to define the collision hitbox. This can be anything that implements [`CanCollide`]
    pub elements: Vec<&'e dyn CanCollide>,
}

impl Default for CollisionContainer<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'e> CollisionContainer<'e> {
    /// Create a new `CollisionContainer`
    #[must_use]
    pub const fn new() -> Self {
        Self { elements: vec![] }
    }

    /// Add an element to the container
    pub fn push(&mut self, element: &'e impl CanCollide) {
        self.elements.push(element);
    }

    /// Returns true if the given element implementing [`CanDraw`] is overlapping the `CollisionContainer`
    pub fn overlaps_element(&self, element: &impl CanDraw) -> bool {
        self.will_overlap_element(element, Vec2D::ZERO)
    }

    /// Returns true if the given element implementing [`CanDraw`] will be overlapping the `CollisionContainer` when moved by `offset`
    pub fn will_overlap_element(&self, element: &impl CanDraw, offset: Vec2D) -> bool {
        PixelContainer::from(element)
            .pixels
            .into_iter()
            .any(|p| self.collides_with_pos(p.pos + offset))
    }
}

impl CanCollide for CollisionContainer<'_> {
    /// Returns true if any of the elements in the `CollisionContainer` intersect the passed position
    fn collides_with_pos(&self, pos: Vec2D) -> bool {
        self.elements.iter().any(|e| e.collides_with_pos(pos))
    }
}

impl<'e> From<&'e dyn CanCollide> for CollisionContainer<'e> {
    fn from(element: &'e dyn CanCollide) -> Self {
        Self {
            elements: vec![element],
        }
    }
}
