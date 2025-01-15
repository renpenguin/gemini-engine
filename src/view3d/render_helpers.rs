use crate::core::Vec2D;

mod index_face;
mod projected_face;
mod projected_vertex;

pub use index_face::IndexFace as Face;
pub(super) use projected_face::ProjectedFace;
pub(super) use projected_vertex::ProjectedVertex;


/// Returns true if the [`Vec2D`]s in the vector are arranged clockwise
#[must_use]
pub fn is_clockwise(points: &[Vec2D]) -> bool {
    let mut m = vec![];
    for i in 0..points.len() {
        let (p1, p2) = (points[i], points[(i + 1) % points.len()]);
        m.push((p1.x - p2.x) * (p1.y + p2.y));
    }

    m.iter().sum::<i64>() <= 0
}
