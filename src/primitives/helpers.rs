use crate::core::Vec2D;

fn is_left_turn(p0: Vec2D, p1: Vec2D, p2: Vec2D) -> bool {
    let v1 = p1 - p0;
    let v2 = p2 - p0;
    v1.perp_dot(v2) > 0
}

fn is_ear(vertex: Vec2D, prev_vertex: Vec2D, next_vertex: Vec2D, polygon: &[Vec2D]) -> bool {
    for i in 0..polygon.len() {
        let p1 = polygon[i];
        let p2 = polygon[(i + 1) % polygon.len()];
        let p3 = polygon[(i + 2) % polygon.len()];

        if p1 != vertex
            && p2 != vertex
            && p3 != vertex
            && is_left_turn(vertex, p1, p2)
            && is_left_turn(vertex, p2, p3)
            && is_left_turn(vertex, p3, p1)
        {
            let triangle_area = (p1 - p2).perp_dot(p3 - p2).abs();
            if triangle_area > 0 {
                let p = (vertex - prev_vertex).perp_dot(next_vertex - prev_vertex);
                return (p1 - prev_vertex).perp_dot(p1 - next_vertex) > 0 && p > 0;
            }
        }
    }
    false
}

/// Split a polygon up into triangles using the ear cutting algorithm. Returns a vec of coordinate sets for each triangle
#[must_use]
pub fn triangulate(vertices: &[Vec2D]) -> Vec<[Vec2D; 3]> {
	let mut triangles = Vec::new();
	let n = vertices.len();

	if n < 3 {
		return triangles;
	}

	let mut remaining_vertices = vertices.to_vec();

	while remaining_vertices.len() > 3 {
		let mut ear_index = 0;
		for i in 0..remaining_vertices.len() {
			let prev_index = (i + remaining_vertices.len() - 1) % remaining_vertices.len();
			let next_index = (i + 1) % remaining_vertices.len();

			let vertex = remaining_vertices[i];
			let prev_vertex = remaining_vertices[prev_index];
			let next_vertex = remaining_vertices[next_index];

			if is_ear(vertex, prev_vertex, next_vertex, &remaining_vertices) {
				ear_index = i;
				break;
			}
		}

		let ear_vertex = remaining_vertices[ear_index];
		let prev_index = (ear_index + remaining_vertices.len() - 1) % remaining_vertices.len();
		let next_index = (ear_index + 1) % remaining_vertices.len();
		let prev_vertex = remaining_vertices[prev_index];
		let next_vertex = remaining_vertices[next_index];

		triangles.push([prev_vertex, ear_vertex, next_vertex]);
		remaining_vertices.remove(ear_index);
	}

	triangles.push([
		remaining_vertices[0],
		remaining_vertices[1],
		remaining_vertices[2],
	]);

	triangles
}

/// Draw a pseudo-line between the independent and dependent positions. Returns rounded values as `isize`s. If you don't want the values rounded, use [`Triangle::interpolate_floating()`]
#[must_use]
pub fn interpolate(i0: i64, d0: i64, i1: i64, d1: i64) -> Vec<i64> {
	interpolate_floating(i0, d0 as f64, i1, d1 as f64)
		.iter()
		.map(|n| n.round() as i64)
		.collect()
}

/// Draw a pseudo-line between the independent and dependent positions
#[must_use]
pub fn interpolate_floating(i0: i64, d0: f64, i1: i64, d1: f64) -> Vec<f64> {
	if i0 == i1 {
		return vec![d0];
	}
	let mut values = vec![];

	let a = (d1 - d0) / (i1 - i0) as f64;
	let mut d = d0;
	for _i in i0..=i1 {
		values.push(d);
		d += a;
	}
	values
}
