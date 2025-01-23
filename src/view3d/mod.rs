//! Gemini's implementation of 3D rendering. Capable of rendering full 3D meshes as wireframes, solid colours or with lighting
//!
//! ## A Simple 3D Scene
//! Let's write a simple example program to print a spinning cube:
//! ```rust,no_run
//! use gemini_engine::elements::{
//!     view::{View, ColChar, Wrapping},
//!     Vec2D,
//! };
//! use gemini_engine::elements3d::{DisplayMode, Mesh3D, Vec3D, Viewport, Transform3D};
//! use gemini_engine::gameloop;
//!
//! const FPS: f32 = 20.0;
//! const FOV: f64 = 95.0;
//!
//! fn main() {
//!     let mut frame_skip = false;
//!     let mut view = View::new(350, 90, ColChar::BACKGROUND);
//!
//!     let mut viewport = Viewport::new(
//!         Transform3D::new_tr(
//!             Vec3D::new(0.0, 0.0, 5.0),
//!             Vec3D::new(-0.5, 0.0, 0.0)
//!         ),
//!         FOV,
//!         view.center(),
//!     );
//!
//!     let cube = Mesh3D::default_cube();
//!
//!     loop {
//!         let now = gameloop::Instant::now();
//!         view.clear();
//!
//!         viewport.transform.rotation.y -= 0.05;
//!
//!         match frame_skip {
//!             true => frame_skip = false,
//!             false => {
//!                 view.blit(
//!                     &viewport.render(vec![&cube], DisplayMode::Solid),
//!                     Wrapping::Ignore
//!                 );
//!                 view.display_render().unwrap();
//!             }
//!         }
//!
//!         let elapsed = now.elapsed();
//!         println!(
//!             "Elapsed: {:.2?}µs | Frame skip: {}",
//!             elapsed.as_micros(),
//!             frame_skip
//!         );
//!
//!         frame_skip = gameloop::sleep_fps(FPS, Some(elapsed));
//!     }
//! }
//! ```
//! There is a lot of code here, but since the main loop is based off of the [`gameloop`](crate::gameloop) principle (Go to the [`gameloop`](crate::gameloop) documentation page to learn more), we'll only focus on the parts that are different from the [`gameloop`](crate::gameloop) example:
//!
//! ### Initialisation
//! ```rust,no_run
//! # use gemini_engine::elements::{View, Vec2D, view::ColChar};
//! # use gemini_engine::elements3d::{Viewport, Mesh3D, Transform3D};
//! # const FOV: f64 = 95.0;
//! let mut view = View::new(350, 90, ColChar::BACKGROUND);
//!
//! let mut viewport = Viewport::new(
//!     Transform3D::DEFAULT,
//!     FOV,
//!     view.size(),
//! );
//!
//! let cube = Mesh3D::default_cube();
//! ```
//! `main()` begins with the creation of all the necessary objects to render 3D images:
//! 1. [`View`](crate::elements::view::View) to handle the canvas and printing to the screen
//! 2. [`Viewport`] to handle converting 3d objects to 2d images, as well as acting like the scene's camera
//! 3. The actual objects you intend to use in the scene, as [`Mesh3D`]
//!
//! In this scenario, we create a [`View`](crate::elements::view::View) of width 350 and height 90 (you may have to zoom out and expand your terminal to fit the whole image), a [`Viewport`] with a transform of rotation 0.5 radians and translation 5 units away from the centre, our desired FOV and origin point (the centre of the view we're printing to) in the middle of the [`View`](crate::elements::view::View) and a single default cube, which is 2 units tall, wide and long and is placed directly in the middle of the scene.
//!
//! ### Gameloop process logic
//! ```rust,no_run
//! # use gemini_engine::elements::{View, Vec2D, view::ColChar};
//! # use gemini_engine::elements3d::{Viewport, Transform3D};
//! # const FOV: f64 = 5000.0;
//! # let view = View::new(350, 90, ColChar::BACKGROUND);
//! # let mut viewport = Viewport::new(
//! #     Transform3D::DEFAULT,
//! #     FOV,
//! #     view.size(),
//! # );
//! viewport.transform.rotation.y -= 0.05;
//! ```
//!
//! This part of the code is where we would put all our physics, collisions, events etc. code, but in this case the only thing we do is rotate the cube 0.05 radians anticlockwise.
//!
//! ### Blitting/Rendering
//! ```rust,no_run
//! # use gemini_engine::elements::{view::{View, ColChar, Wrapping}, Vec2D};
//! # use gemini_engine::elements3d::{Viewport, Mesh3D, DisplayMode, Transform3D};
//! # const FOV: f64 = 5000.0;
//! # let mut view = View::new(350, 90, ColChar::BACKGROUND);
//! # let viewport = Viewport::new(
//! #     Transform3D::DEFAULT,
//! #     FOV,
//! #     view.size(),
//! # );
//! # let cube = Mesh3D::default_cube();
//! view.blit(&viewport.render(vec![&cube], DisplayMode::Solid), Wrapping::Ignore);
//! view.display_render().unwrap();
//! ```
//!
//! This part of the code renders all the 3d stuff to the [`View`](crate::elements::view::View) and blits it to the view before rendering as usual. [`Viewport.render()`](Viewport) takes a list of all the objects we want to render and a [`DisplayMode`] enum (more info in the [`DisplayMode`] documentation).

use crate::{
    core::{CanDraw, ColChar, Vec2D},
    mesh3d::{Mesh3D, Transform3D},
    primitives::{Line, Polygon},
};
use glam::DVec2;

mod display_mode;
mod projected_face;

pub use display_mode::{
    lighting::{Light, LightType, BRIGHTNESS_CHARS},
    DisplayMode,
};
use projected_face::{ProjectedFace, ProjectedVertex};

/// The `Viewport` handles printing 3D objects to a 2D [`View`](crate::elements::View), and also acts as the scene's camera.
pub struct Viewport {
    /// How the Viewport is oriented in the 3D scene
    pub camera_transform: Transform3D,
    /// The Viewport's field of view, in degrees
    pub fov: f64,
    /// The centre of the view you intend to draw to. [`View.size()`](crate::view::View::center) returns exactly what you need for this
    pub canvas_centre: Vec2D,
    pub objects: Vec<Mesh3D>,
    pub display_mode: DisplayMode,
    /// Most terminals don't have perfectly square characters. The value you set here is how much the final image will be stretched in the X axis to account for this. The default value is `2.2` but it will be different in most terminals
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

    /// Transform the vertices with the object transform, view transform and perspective transform
    fn get_vertices_on_screen(&self, object: &Mesh3D) -> Vec<ProjectedVertex> {
        let perspective =
            Transform3D::perspective_infinite_rh(self.fov.to_radians(), 1.0, self.clipping_distace);
        let centre = DVec2::new(self.canvas_centre.x as f64, self.canvas_centre.y as f64);
        let size = DVec2::splat(centre.min_element());
        object
            .vertices
            .iter()
            .map(|v| object.transform.transform_point3(*v)) // Object transform
            .map(|v| self.camera_transform.transform_point3(v)) // Camera transform
            .map(|v| (v, perspective.project_point3(v))) // Perspective
            .map(|(v, pv)| {
                (
                    v,
                    DVec2::new(pv.x * self.character_width_multiplier, -pv.y) * size + centre,
                )
            }) // Map to screen
            .map(|(v, pv)| ProjectedVertex::new(v, Vec2D::new(pv.x as i64, pv.y as i64)))
            .collect()
    }

    /// Project the faces onto a 2D plane. Returns a collection of faces, each stored as a list of the points it appears at, the normal of the face and the [`ColChar`] assigned to it
    fn project_faces(&self, sort_faces: bool, backface_culling: bool) -> Vec<ProjectedFace> {
        let mut screen_faces = vec![];

        for object in &self.objects {
            let vertices = self.get_vertices_on_screen(object);

            for face in &object.faces {
                let face_vertices = face.index_into(&vertices);

                // Backface culling
                if backface_culling && !projected_face::is_clockwise(&face_vertices) {
                    continue;
                }

                // Do not render if behind player
                if face_vertices
                    .iter()
                    .any(|v| v.original.z <= self.clipping_distace)
                {
                    continue;
                }

                screen_faces.push(ProjectedFace::new(
                    face_vertices,
                    face.fill_char,
                ));
            }
        }

        if sort_faces {
            screen_faces
                .sort_by_key(|face| (face.get_average_centre().length() * -1000.0).round() as isize);
        }

        screen_faces
    }
}

impl CanDraw for Viewport {
    fn draw_to(&self, canvas: &mut impl crate::core::Canvas) {
        match &self.display_mode {
            DisplayMode::Wireframe { backface_culling } => {
                let screen_faces = self.project_faces(false, *backface_culling);

                for face in screen_faces {
                    for fi in 0..face.vertices.len() {
                        let (i0, i1) = (
                            face.vertices[fi].projected,
                            face.vertices[(fi + 1) % face.vertices.len()].projected,
                        );
                        Line::new(i0, i1, face.fill_char).draw_to(canvas);
                    }
                }
            }
            DisplayMode::Solid => {
                let screen_faces = self.project_faces(true, true);

                for face in screen_faces {
                    Polygon::new(&face.projected_vertices(), face.fill_char).draw_to(canvas);
                }
            }
            DisplayMode::Illuminated { lights } => {
                let screen_faces = self.project_faces(true, true);

                let brightness_chars: Vec<char> = BRIGHTNESS_CHARS.chars().collect();
                let len_brightness_chars: f64 = brightness_chars.len() as f64;

                for face in screen_faces {
                    let fill_char = if let Some(normal) = face.get_normal() {
                        let intensity: f64 = lights
                            .iter()
                            .map(|light| {
                                light.calculate_intensity(face.get_average_centre(), normal)
                            })
                            .sum();

                        let brightness_char_index = ((intensity * len_brightness_chars).round()
                            as usize)
                            .clamp(0, brightness_chars.len() - 1);
                        let intensity_char = brightness_chars[brightness_char_index];

                        ColChar::new(intensity_char, face.fill_char.modifier)
                    } else {
                        face.fill_char
                    };

                    Polygon::new(&face.projected_vertices(), fill_char).draw_to(canvas);
                }
            }
        }
    }
}
