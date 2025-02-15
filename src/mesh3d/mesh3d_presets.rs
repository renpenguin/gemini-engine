//! This file contains the presets available when spawning a [`Mesh3D`]

use crate::core::{ColChar, Modifier};
use std::f64::consts::TAU;

use super::{Face, Mesh3D, Transform3D, Vec3D};

impl Mesh3D {
    /// The `gemini_engine` equivalent of Blender's default cube. Has sides of length 2
    #[must_use]
    pub fn default_cube() -> Self {
        Self::new(
            vec![
                Vec3D::new(1.0, 1.0, -1.0),
                Vec3D::new(1.0, 1.0, 1.0),
                Vec3D::new(1.0, -1.0, -1.0),
                Vec3D::new(1.0, -1.0, 1.0),
                Vec3D::new(-1.0, 1.0, -1.0),
                Vec3D::new(-1.0, 1.0, 1.0),
                Vec3D::new(-1.0, -1.0, -1.0),
                Vec3D::new(-1.0, -1.0, 1.0),
            ],
            vec![
                Face::new(vec![2, 3, 1, 0], ColChar::SOLID.with_mod(Modifier::BLUE)),
                Face::new(vec![4, 5, 7, 6], ColChar::SOLID.with_mod(Modifier::BLUE)),
                Face::new(vec![1, 3, 7, 5], ColChar::SOLID.with_mod(Modifier::None)),
                Face::new(vec![4, 6, 2, 0], ColChar::SOLID.with_mod(Modifier::None)),
                Face::new(vec![6, 7, 3, 2], ColChar::SOLID.with_mod(Modifier::RED)),
                Face::new(vec![0, 1, 5, 4], ColChar::SOLID.with_mod(Modifier::RED)),
            ],
        )
    }

    /// Create a torus (donut shape)
    #[must_use]
    pub fn torus(
        outer_radius: f64,
        inner_radius: f64,
        outer_segments: usize,
        inner_segments: usize,
    ) -> Self {
        let mut vertices = vec![];
        let mut faces = vec![];

        for outer_i in 0..outer_segments {
            let outer_angle = (outer_i as f64 / outer_segments as f64) * TAU;
            let outer_transform = Transform3D::from_rotation_y(-outer_angle);
            let outer_point = Vec3D::new(
                outer_angle.cos() * outer_radius,
                0.0,
                outer_angle.sin() * outer_radius,
            );

            for inner_i in 0..inner_segments {
                let inner_angle = (inner_i as f64 / inner_segments as f64) * TAU;
                let inner_point = Vec3D::new(
                    inner_angle.cos() * inner_radius,
                    inner_angle.sin() * inner_radius,
                    0.0,
                );
                vertices.push(outer_point + outer_transform.transform_vector3(inner_point));

                let inc_outer_i = (outer_i + 1) % outer_segments;
                let inc_inner_i = (inner_i + 1) % inner_segments;
                faces.push(Face::new(
                    vec![
                        inc_outer_i * inner_segments + inner_i,
                        inc_outer_i * inner_segments + inc_inner_i,
                        outer_i * inner_segments + inc_inner_i,
                        outer_i * inner_segments + inner_i,
                    ],
                    ColChar::SOLID,
                ));
            }
        }

        Self::new(vertices, faces)
    }

    /// A gimbal to help you orient in `gemini_engine`'s 3D space. The orientation is as follows (from the default [`Viewport`](crate::view3d::Viewport))
    /// - X (red) increases as you move to the right
    /// - Y (green) increases as you move up
    /// - Z (blue) increases as you move away from the viewport
    ///
    /// Think of it like Blender's axes but with Y and Z swapped.
    /// Since this Mesh does not have a real triangle mesh, it is only visible in [`DisplayMode::Wireframe`](crate::view3d::DisplayMode)
    #[must_use]
    pub fn gimbal() -> Self {
        Self::new(
            vec![
                Vec3D::ZERO,
                Vec3D::new(1.0, 0.0, 0.0),
                Vec3D::new(0.0, 1.0, 0.0),
                Vec3D::new(0.0, 0.0, 1.0),
            ],
            vec![
                Face::new(vec![0, 1], ColChar::SOLID.with_mod(Modifier::RED)),
                Face::new(vec![0, 2], ColChar::SOLID.with_mod(Modifier::GREEN)),
                Face::new(vec![0, 3], ColChar::SOLID.with_mod(Modifier::BLUE)),
            ],
        )
    }
}
