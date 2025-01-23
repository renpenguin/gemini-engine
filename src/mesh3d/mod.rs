//! This module contains the [`Mesh3D`], which stores 3D objects as vertices and index faces

use crate::view3d::Face;
mod mesh3d_presets;

/// An alias to [`DVec3`](glam::DVec3), a three-dimensional vector of `f64` values
pub type Vec3D = glam::DVec3;
/// An alias to [`DMat4`](glam::DMat4), a 4x4 matrix of `f64` values
pub type Transform3D = glam::DMat4;

/// A 3D mesh made up of vertices, faces made of indices into `vertices`, and a transformation.
#[derive(Debug, Clone)]
pub struct Mesh3D {
    /// The mesh's transform (position, rotation, scale) in 3D space
    pub transform: Transform3D,
    /// A vector of the [`Mesh3D`]'s
    pub vertices: Vec<Vec3D>,
    /// A vector of [`Face`]s of indexes into `vertices`
    pub faces: Vec<Face>,
}

impl Mesh3D {
    /// Create a `Mesh3D` with an identity `Transform3D`
    #[must_use]
    pub const fn new(vertices: Vec<Vec3D>, faces: Vec<Face>) -> Self {
        Self {
            transform: Transform3D::IDENTITY,
            vertices,
            faces,
        }
    }

    /// Return the `Mesh3D` with an updated `transform` property. Consumes the original `Mesh3D`
    #[must_use]
    pub const fn with_transform(mut self, transform: Transform3D) -> Self {
        self.transform = transform;
        self
    }
}
