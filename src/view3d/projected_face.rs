use crate::{
    core::{ColChar, Vec2D},
    mesh3d::Vec3D,
};

/// Returns true if the [`ProjectedVertex`]s in the slice are arranged clockwise
#[must_use]
pub fn is_clockwise(points: &[ProjectedVertex]) -> bool {
    if points.len() < 3 {
        return false;
    }
    let mut m = vec![];
    for i in 0..points.len() {
        let (p1, p2) = (
            points[i].projected,
            points[(i + 1) % points.len()].projected,
        );
        m.push((p1.x - p2.x) * (p1.y + p2.y));
    }

    m.iter().sum::<i64>() <= 0
}

#[derive(Debug, Clone, Copy)]
pub struct ProjectedVertex {
    pub original: Vec3D,
    pub projected: Vec2D,
}

impl ProjectedVertex {
    pub const fn new(original: Vec3D, projected: Vec2D) -> Self {
        Self {
            original,
            projected,
        }
    }
}

pub struct ProjectedFace {
    /// The face's points, both in 3D and projected 2D space
    pub vertices: Vec<ProjectedVertex>,
    /// The face's fill [`ColChar`]
    pub fill_char: ColChar,
}

impl ProjectedFace {
    /// Create a new `ProjectedFace`
    pub const fn new(vertices: Vec<ProjectedVertex>, fill_char: ColChar) -> Self {
        Self {
            vertices,
            fill_char,
        }
    }

    pub fn projected_vertices(&self) -> Vec<Vec2D> {
        self.vertices.iter().map(|v| v.projected).collect()
    }

    /// Get the "centre" of the face in 3D space, calculated based on the average of the original vertices
    pub fn get_average_centre(&self) -> Vec3D {
        self.vertices.iter().map(|v| v.original).sum::<Vec3D>() / self.vertices.len() as f64
    }

    /// Get the normal of the face
    pub fn get_normal(&self) -> Option<Vec3D> {
        if self.vertices.len() < 3 {
            return None;
        }
        let v0 = self.vertices[0].original - self.vertices[2].original;
        let v1 = self.vertices[1].original - self.vertices[2].original;
        Some(v0.cross(v1).normalize())
    }
}
