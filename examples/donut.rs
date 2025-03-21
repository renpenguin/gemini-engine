//! A visual recreation of donut-c in gemini
use gemini_engine::{
    core::ColChar,
    mesh3d::{Mesh3D, Transform3D, Vec3D},
    view::View,
    view3d::{DisplayMode, Light, Viewport},
};
use std::{thread, time::Duration};

const FPS: f32 = 60.0;
const FOV: f64 = 95.0;

fn main() {
    let mut view = View::new(100, 45, ColChar::EMPTY);
    let mut viewport = Viewport::new(
        Transform3D::look_at_lh(Vec3D::new(0.0, -3.0, 6.0), Vec3D::ZERO, Vec3D::Y),
        FOV,
        view.center(),
    );

    viewport.display_mode = DisplayMode::Illuminated {
        lights: vec![
            Light::new_ambient(0.3),
            Light::new_directional(0.7, Vec3D::new(1.0, 1.0, 1.0)),
        ],
    };

    viewport.objects.push(Mesh3D::torus(1.8, 1.0, 32, 16));

    loop {
        let donut_tr = &mut viewport.objects[0].transform;
        *donut_tr = donut_tr.mul_mat4(&Transform3D::from_rotation_y(-0.03));
        *donut_tr = donut_tr.mul_mat4(&Transform3D::from_rotation_x(0.03));

        view.clear();
        view.draw(&viewport);
        let _ = view.display_render();

        thread::sleep(Duration::from_secs_f32(1.0 / FPS));
    }
}
