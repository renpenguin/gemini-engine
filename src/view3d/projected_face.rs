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
    pub vertices: Vec<Vec2D>,
    /// The normal vector of the face
    pub normal: Option<Vec3D>,
    /// The distance from the camera
    pub original_centre: Vec3D,
    /// The face's fill [`ColChar`]
    pub fill_char: ColChar,
}

impl ProjectedFace {
    pub fn new(vertices: Vec<ProjectedVertex>, fill_char: ColChar) -> Self {
        let len = vertices.len();
        let (original_vertices, vertices): (Vec<_>, Vec<_>) = vertices
            .into_iter()
            .map(|v| (v.original, v.projected))
            .unzip();
        Self {
            vertices,
            normal: (len >= 3).then(|| {
                let v0 = original_vertices[0] - original_vertices[2];
                let v1 = original_vertices[1] - original_vertices[2];
                v0.cross(v1).normalize()
            }),
            original_centre: original_vertices.into_iter().sum::<Vec3D>() / len as f64,
            fill_char,
        }
    }
}
