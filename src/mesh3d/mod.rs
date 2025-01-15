use crate::view3d::Face;
mod mesh3d_presets;

pub type Vec3D = glam::DVec3;
pub type Transform3D = glam::DMat4;

/// The struct for a `Mesh3D` object, containing a position, rotation, collection of vertices and collection of [`Face`]s with indices to the vertex collection.
#[derive(Debug, Clone)]
pub struct Mesh3D {
    /// The mesh's transform (position, rotation, scale) in 3D space
    pub transform: Transform3D,
    /// A vector of the [`Mesh3D`]'s
    pub vertices: Vec<Vec3D>,
    /// A vector of [`Face`]s of indexes into [`Mesh3D::vertices`]
    pub faces: Vec<Face>,
}

impl Mesh3D {
    /// Create a `Mesh3D` with a default `Transform3D`
    #[must_use]
    pub const fn new(vertices: Vec<Vec3D>, faces: Vec<Face>) -> Self {
        Self {
            transform: Transform3D::IDENTITY,
            vertices,
            faces,
        }
    }

    /// Create a `Mesh3D` with `Transform3D` set to an identity matrix
    #[must_use]
    pub const fn with_transform(mut self, transform: Transform3D) -> Self {
        self.transform = transform;
        self
    }
}
