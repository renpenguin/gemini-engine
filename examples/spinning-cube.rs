//! An example of a spinning cube with `view3d`
use gemini_engine::{
    core::ColChar,
    fps_gameloop,
    mesh3d::{Mesh3D, Transform3D, Vec3D},
    view::View,
    view3d::{DisplayMode, Light, Viewport},
};
use std::time::Duration;

const FPS: f32 = 30.0;
const FOV: f64 = 80.0;

fn main() {
    let mut view = View::new(100, 50, ColChar::EMPTY);

    let mut viewport = Viewport::new(
        Transform3D::look_at_lh(Vec3D::new(0.0, -1.5, 4.3), Vec3D::ZERO, Vec3D::Y),
        FOV,
        view.center(),
    );
    viewport.objects.push(Mesh3D::default_cube());

    viewport.display_mode = DisplayMode::Illuminated {
        lights: vec![
            Light::new_ambient(0.3),
            Light::new_directional(0.6, Vec3D::new(0.5, 1.0, 1.0)),
        ],
    };

    fps_gameloop!(
        {
            viewport.objects[0].transform = viewport.objects[0]
                .transform
                .mul_mat4(&Transform3D::from_rotation_y(-0.05));
        },
        {
            view.clear();
            view.draw(&viewport);
            let _ = view.display_render();
        },
        FPS,
        |elapsed: Duration, frame_skip| {
            println!(
                "Elapsed: {:.2?}µs | Frame skip: {}",
                elapsed.as_micros(),
                frame_skip
            );
        }
    );
}
