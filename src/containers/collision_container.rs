use crate::{
    containers::PixelContainer,
    core::{CanDraw, Vec2D},
};

/// Must be implemented to be used by the [`CollisionContainer`]
pub trait CanCollide {
    /// Return `true` if the collider collides with the passed new position
    #[must_use]
    fn collides_with_pos(&self, pos: Vec2D) -> bool;
}

/// Contains references to all added objects. Meant to be used specifically for collision calculations
#[derive(Clone)]
pub struct CollisionContainer<'a> {
    /// The elements used to define the collision hitbox. This can be anything that implements [`CanCollide`]
    pub elements: Vec<&'a dyn CanCollide>,
}

impl<'a> Default for CollisionContainer<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> CollisionContainer<'a> {
    /// Create a new `CollisionContainer`
    #[must_use]
    pub const fn new() -> Self {
        Self { elements: vec![] }
    }

    /// Add an element to the container
    pub fn push(&mut self, element: &'a impl CanCollide) {
        self.elements.push(element);
    }

    /// Returns true if the given [`CanCollide`] is overlapping the `CollisionContainer`
    pub fn overlaps_element(&self, element: &impl CanDraw) -> bool {
        self.will_overlap_element(element, Vec2D::ZERO)
    }

    /// Returns true if the element will be overlapping the `CollisionContainer` when the offset is applied
    pub fn will_overlap_element(&self, element: &impl CanDraw, offset: Vec2D) -> bool {
        PixelContainer::from(element)
            .pixels
            .into_iter()
            .any(|p| self.collides_with_pos(p.pos + offset))
    }
}

impl<'a> CanCollide for CollisionContainer<'a> {
    /// Returns true if there is an element from the `CollisionContainer` at the given coordinates
    fn collides_with_pos(&self, pos: Vec2D) -> bool {
        self.elements.iter().any(|e| e.collides_with_pos(pos))
    }
}

impl<'a> From<&[&'a dyn CanCollide]> for CollisionContainer<'a> {
    fn from(elements: &[&'a dyn CanCollide]) -> Self {
        Self {
            elements: elements.to_vec(),
        }
    }
}
