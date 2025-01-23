use crate::core::ColChar;

/// An alias to [`DVec3`](glam::DVec3), a three-dimensional vector of `f64` values
pub type Vec3D = glam::DVec3;
/// An alias to [`DMat4`](glam::DMat4), a 4x4 matrix of `f64` values
pub type Transform3D = glam::DMat4;

/// A `Face` contains indices to a mesh's collection of vertices and a `ColChar` to fill the face. Indices should be arranged in a clockwise order, as if they appear counter-clockwise when rendering they will not be rendered at all (this is how gemini-engine handles backface culling and maximises performance)
#[derive(Debug, Clone)]
pub struct Face {
    /// The vertex indices of the face
    pub v_indices: Vec<usize>,
    /// The desired appearance of the face when rendered
    pub fill_char: ColChar,
}

impl Face {
    /// Create a new face with the given indices and [`ColChar`]
    #[must_use]
    pub const fn new(v_indices: Vec<usize>, fill_char: ColChar) -> Self {
        Self {
            v_indices,
            fill_char,
        }
    }

    /// Return a vector with the elements found at the vertex indices of the given slice
    pub fn index_into<T: Copy>(&self, vertices: &[T]) -> Vec<T> {
        // TODO: return `None` if the input slice isnt large enough
        self.v_indices.iter().map(|vi| vertices[*vi]).collect()
    }
}
