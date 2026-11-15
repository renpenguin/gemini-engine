//! Gemini's implementation of 3D rendering. Capable of rendering full 3D meshes as wireframes, solid colours or with lighting
//!
//! ## Example
//! Let's write a simple example program to draw a spinning cube. This example is available in `examples/spinning-cube.rs`
//! ```no_run
#![doc = include_str!("../../examples/spinning-cube.rs")]
//! ```
//! There is a lot of code here, but here we'll only focus on the parts that are different from the [`gameloop`](crate::gameloop) example:
//!
//! ### Initialisation
//! ```no_run
//! # use gemini_engine::{core::{ColChar, Vec2D}, view::View};
//! # use gemini_engine::{view3d::{Viewport, Light, DisplayMode}, mesh3d::{Mesh3D, Transform3D, Vec3D}};
//! # const FOV: f64 = 95.0;
//! let mut view = View::new(100, 50, ColChar::EMPTY);
//!
//! let mut viewport = Viewport::new(
//!     Transform3D::look_at_lh(Vec3D::new(0.0, -1.5, 4.3), Vec3D::ZERO, Vec3D::Y),
//!     FOV,
//!     view.center(),
//! );
//! viewport.objects.push(Mesh3D::default_cube());
//!
//! viewport.display_mode = DisplayMode::Illuminated {
//!     lights: vec![
//!         Light::new_ambient(0.3),
//!         Light::new_directional(0.6, Vec3D::new(0.5, 1.0, 1.0)),
//!     ],
//! };
//! ```
//! `main()` begins with the creation of all the necessary objects to render 3D images:
//! 1. [`View`](crate::view::View) to handle the canvas and printing to the screen
//! 2. [`Viewport`] to handle drawing 3d objects to the canvas
//! 3. The actual objects you intend to use in the scene, as [`Mesh3D`]
//!
//! In this scenario, we create a [`View`](crate::view::View) of width 100 and height 50 (you may have to zoom out and expand your terminal to fit the whole image), a [`Viewport`] with a camera positioned at (0,-1.5,4.3) pointing at (0,0,0), our desired FOV and origin point (the centre of the view we're printing to) in the middle of the [`View`](crate::view::View). We add a single default cube, which is 2 units tall, wide and long to the `Viewport`s list of objects, and set the `Viewport`'s `display_mode` to [`DisplayMode::Illuminated`] with a simple lighting setup
//!
//! ### Gameloop process logic
//! ```no_run
//! # use gemini_engine::{view::View, core::{Vec2D, ColChar}};
//! # use gemini_engine::{view3d::Viewport, mesh3d::Transform3D};
//! # const FOV: f64 = 5000.0;
//! # let view = View::new(350, 90, ColChar::BACKGROUND);
//! # let mut viewport = Viewport::new(
//! #     Transform3D::IDENTITY,
//! #     FOV,
//! #     view.size(),
//! # );
//! viewport.objects[0].transform = viewport.objects[0]
//!     .transform
//!     .mul_mat4(&Transform3D::from_rotation_y(-0.05));
//! ```
//!
//! This part of the code is where we would put all our physics, collisions, events etc. code, but in this case the only thing we do is rotate the cube 0.05 radians anticlockwise in the Y axis.
//!
//! ### Drawing/Rendering
//! ```no_run
//! # use gemini_engine::{core::{Vec2D, ColChar}, view::View};
//! # use gemini_engine::view3d::{Viewport, DisplayMode};
//! # use gemini_engine::mesh3d::{Mesh3D, Transform3D};
//! # const FOV: f64 = 5000.0;
//! # let mut view = View::new(350, 90, ColChar::BACKGROUND);
//! # let viewport = Viewport::new(
//! #     Transform3D::IDENTITY,
//! #     FOV,
//! #     view.size(),
//! # );
//! view.clear();
//! view.draw(&viewport);
//! let _ = view.display_render();
//! ```
//!
//! This part of the code renders and draws all the 3d stuff to the [`View`](crate::view::View) before rendering with `display_render` as usual. [`Viewport`] implements [`CanDraw`], so when it is draw to the `View`, it fully renders our scene based on its stored transform, display mode, objects, etc.

use crate::{
    core::{CanDraw, Vec2D},
    mesh3d::{Mesh3D, Transform3D, Vec3D},
    primitives::{Line, Polygon},
};
use glam::DVec2;

mod display_mode;
mod projected_face;

pub use display_mode::{
    DisplayMode,
    lighting::{BRIGHTNESS_CHARS, Light, LightType},
};
use projected_face::ProjectedFace;

/// The `Viewport` handles drawing 3D objects to a 2D [`Canvas`](crate::core::Canvas), and also acts as the scene's camera.
pub struct Viewport {
    /// This transform is applied to every vertex in the scene. [`Transform3D::look_at_lh`] works best for this
    pub camera_transform: Transform3D,
    /// The Viewport's field of view, in degrees
    pub fov: f64,
    /// The centre of the view you intend to draw to. [`View.centre()`](crate::view::View::center) returns exactly what you need for this
    pub canvas_centre: Vec2D,
    /// The objects to be drawn on the screen
    pub objects: Vec<Mesh3D>,
    /// The style in which the objects should be rendered. Read [`DisplayMode`] for more info
    pub display_mode: DisplayMode,
    /// Most terminals don't have perfectly square characters. The value you set here is how much the final image will be stretched in the X axis to account for this. The default value is `2.0` but it will be different in most terminals
    pub character_width_multiplier: f64,
    /// Any face with vertices closer to the viewport than this value will be clipped
    pub clipping_distace: f64,
}

impl Viewport {
    /// Create a new `Viewport`
    #[must_use]
    pub const fn new(camera_transform: Transform3D, fov: f64, canvas_centre: Vec2D) -> Self {
        Self {
            camera_transform,
            fov,
            canvas_centre,
            objects: Vec::new(),
            display_mode: DisplayMode::Solid,
            character_width_multiplier: 2.0,
            clipping_distace: 0.3,
        }
    }

    /// Transform the vertices with the object transform and view transform, into camera space
    fn get_vertices_in_camera_space(&self, object: &Mesh3D) -> Vec<Vec3D> {
        let world_transform = self.camera_transform.mul_mat4(&object.transform);

        object
            .vertices
            .iter()
            .map(|v| world_transform.transform_point3(*v))
            .collect()
    }

    /// Project a camera space point onto the canvas with the perspective transform
    fn project_point(&self, perspective: &Transform3D, v: Vec3D) -> Vec2D {
        let centre = DVec2::new(self.canvas_centre.x as f64, self.canvas_centre.y as f64);
        let size = DVec2::splat(centre.max_element());

        let pv = perspective.project_point3(v);
        let pv = DVec2::new(pv.x * self.character_width_multiplier, -pv.y) * size + centre;
        Vec2D::new(pv.x as i64, pv.y as i64)
    }

    /// Project the models' faces onto a 2D plane. Returns a collection of `ProjectedFace`s, each storing its projected vertices, normal and z index
    fn project_faces(&self, sort_faces: bool, backface_culling: bool) -> Vec<ProjectedFace> {
        let mut screen_faces = vec![];
        let perspective =
            Transform3D::perspective_infinite_rh(self.fov.to_radians(), 1.0, self.clipping_distace);

        for object in &self.objects {
            let vertices = self.get_vertices_in_camera_space(object);
            for face in &object.faces {
                let face_vertices = face
                    .index_into(&vertices)
                    .expect("Failed to index mesh vertices with face indices");

                // Do not render the parts of the face that are behind the camera
                let visible_vertices = if face_vertices.iter().all(|v| v.z > self.clipping_distace)
                {
                    face_vertices.clone()
                } else {
                    clip_to_near_plane(&face_vertices, self.clipping_distace)
                };
                if visible_vertices.len() < 3 {
                    continue; // Entirely behind the camera
                }

                let projected: Vec<Vec2D> = visible_vertices
                    .into_iter()
                    .map(|v| self.project_point(&perspective, v))
                    .collect();

                if backface_culling && !projected_face::is_clockwise(&projected) {
                    continue; // Backface culling
                }

                screen_faces.push(ProjectedFace::new(
                    &face_vertices,
                    projected,
                    face.fill_char,
                ));
            }
        }

        if sort_faces {
            screen_faces
                .sort_by_key(|face| (face.original_centre.length() * -1000.0).round() as isize);
        }

        screen_faces
    }
}

/// Clip a camera space polygon against the near plane `z = near`, keeping the part in front of it (Sutherland-Hodgman). Returns fewer than three vertices if nothing is left
fn clip_to_near_plane(polygon: &[Vec3D], near: f64) -> Vec<Vec3D> {
    let mut clipped = Vec::with_capacity(polygon.len() + 1);
    for (i, &a) in polygon.iter().enumerate() {
        let b = polygon[(i + 1) % polygon.len()];
        let (da, db) = (a.z - near, b.z - near);
        if da >= 0.0 {
            clipped.push(a);
        }
        if (da >= 0.0) != (db >= 0.0) {
            clipped.push(a.lerp(b, da / (da - db)));
        }
    }
    clipped
}

impl CanDraw for Viewport {
    /// Project the `models` and draw them onto a [`Canvas`](crate::core::Canvas)
    fn draw_to(&self, canvas: &mut impl crate::core::Canvas) {
        match &self.display_mode {
            DisplayMode::Wireframe { backface_culling } => {
                let screen_faces = self.project_faces(false, *backface_culling);

                for face in screen_faces {
                    for fi in 0..face.vertices.len() {
                        Line::new(
                            face.vertices[fi],
                            face.vertices[(fi + 1) % face.vertices.len()],
                            face.fill_char,
                        )
                        .draw_to(canvas);
                    }
                }
            }
            DisplayMode::Solid => {
                let screen_faces = self.project_faces(true, true);

                for face in screen_faces {
                    Polygon::new(&face.vertices, face.fill_char).draw_to(canvas);
                }
            }
            DisplayMode::Illuminated { lights } => {
                let screen_faces = self.project_faces(true, true);

                let brightness_chars: Vec<char> = BRIGHTNESS_CHARS.chars().collect();
                let len_brightness_chars: f64 = brightness_chars.len() as f64;

                for face in screen_faces {
                    let Some(normal) = face.normal else {
                        continue;
                    };

                    let intensity: f64 = lights
                        .iter()
                        .map(|light| light.calculate_intensity(face.original_centre, normal))
                        .sum();

                    let brightness_char_index = ((intensity * len_brightness_chars).round()
                        as usize)
                        .clamp(0, brightness_chars.len() - 1);
                    let intensity_char = brightness_chars[brightness_char_index];

                    Polygon::new(&face.vertices, face.fill_char.with_char(intensity_char))
                        .draw_to(canvas);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{core::ColChar, mesh3d::Face};

    /// A canvas that records every point plotted to it
    struct Recorder(Vec<Vec2D>);

    impl crate::core::Canvas for Recorder {
        fn plot(&mut self, pos: Vec2D, _c: ColChar) {
            self.0.push(pos);
        }
    }

    /// A camera at the origin looking down +Z, set to draw one quad as a wireframe with backface culling disabled, so culling cannot hide any face
    fn viewport_with_quad(corners: [Vec3D; 4]) -> Viewport {
        let mut viewport = Viewport::new(
            Transform3D::look_at_lh(Vec3D::ZERO, Vec3D::Z, Vec3D::Y),
            90.0,
            Vec2D::new(50, 25),
        );
        viewport.objects.push(Mesh3D::new(
            corners.to_vec(),
            vec![Face::new(vec![0, 1, 2, 3], ColChar::SOLID)],
        ));
        viewport.display_mode = DisplayMode::Wireframe {
            backface_culling: false,
        };
        viewport
    }

    fn draw(viewport: &Viewport) -> Vec<Vec2D> {
        let mut recorder = Recorder(Vec::new());
        viewport.draw_to(&mut recorder);
        recorder.0
    }

    #[test]
    fn faces_behind_the_camera_are_not_drawn() {
        let viewport = viewport_with_quad([
            Vec3D::new(-1.0, -1.0, -5.0),
            Vec3D::new(1.0, -1.0, -5.0),
            Vec3D::new(1.0, 1.0, -5.0),
            Vec3D::new(-1.0, 1.0, -5.0),
        ]);
        let plotted = draw(&viewport);
        assert!(plotted.is_empty(), "drew {} points", plotted.len());
    }

    #[test]
    fn faces_crossing_the_near_plane_are_clipped_not_mirrored() {
        // A plane one unit off the camera's axis, reaching from behind the camera to far in front of it
        let viewport = viewport_with_quad([
            Vec3D::new(-1.0, -1.0, -5.0),
            Vec3D::new(1.0, -1.0, -5.0),
            Vec3D::new(1.0, -1.0, 20.0),
            Vec3D::new(-1.0, -1.0, 20.0),
        ]);
        let plotted = draw(&viewport);
        assert_ne!(plotted.len(), 0);

        // Every point of the visible part lies on the same side of the horizon as a point of the plane that is in front of the camera. The part behind the camera, if projected, would be mirrored onto the other side
        let perspective = Transform3D::perspective_infinite_rh(
            viewport.fov.to_radians(),
            1.0,
            viewport.clipping_distace,
        );
        let horizon = viewport.canvas_centre.y;
        let side = (viewport
            .project_point(&perspective, Vec3D::new(0.0, -1.0, 5.0))
            .y
            - horizon)
            .signum();
        assert_ne!(side, 0);
        let mirrored = plotted
            .iter()
            .filter(|p| (p.y - horizon).signum() == -side)
            .count();
        assert_eq!(
            mirrored, 0,
            "{mirrored} points on the wrong side of the horizon"
        );
    }

    #[test]
    fn near_plane_clipping_keeps_only_the_part_in_front() {
        let square = [
            Vec3D::new(-1.0, 0.0, -1.0),
            Vec3D::new(1.0, 0.0, -1.0),
            Vec3D::new(1.0, 0.0, 3.0),
            Vec3D::new(-1.0, 0.0, 3.0),
        ];
        let clipped = clip_to_near_plane(&square, 1.0);
        assert_eq!(clipped.len(), 4);
        assert!(clipped.iter().all(|v| v.z >= 1.0));
        assert!(clipped.contains(&Vec3D::new(1.0, 0.0, 1.0)));
        assert!(clipped.contains(&Vec3D::new(-1.0, 0.0, 3.0)));

        assert!(clip_to_near_plane(&square, 5.0).len() < 3, "fully behind");
        assert_eq!(clip_to_near_plane(&square, -2.0), square, "fully in front");
    }
}
