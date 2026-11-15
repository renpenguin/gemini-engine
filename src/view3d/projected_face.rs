use crate::{
    core::{ColChar, Vec2D},
    mesh3d::Vec3D,
};

/// Returns true if the projected points in the slice are arranged clockwise
#[must_use]
pub fn is_clockwise(points: &[Vec2D]) -> bool {
    if points.len() < 3 {
        return false;
    }
    let mut m = vec![];
    for i in 0..points.len() {
        let (p1, p2) = (points[i], points[(i + 1) % points.len()]);
        m.push((p1.x - p2.x) * (p1.y + p2.y));
    }

    m.iter().sum::<i64>() <= 0
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
    /// `original` is the whole face in camera space, used for its normal and centre. `projected` is the part of the face in front of the camera, projected onto the canvas
    pub fn new(original: &[Vec3D], projected: Vec<Vec2D>, fill_char: ColChar) -> Self {
        let len = original.len();
        Self {
            vertices: projected,
            normal: (len >= 3).then(|| {
                let v0 = original[0] - original[2];
                let v1 = original[1] - original[2];
                v0.cross(v1).normalize()
            }),
            original_centre: original.iter().sum::<Vec3D>() / len as f64,
            fill_char,
        }
    }
}
