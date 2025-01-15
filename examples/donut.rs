//! A visual recreation of donut-c in gemini
use gemini_engine::{
    core::ColChar,
    fps_gameloop,
    mesh3d::Mesh3D,
    view::View,
    view3d::{DisplayMode, Light, Transform3D, Vec3D, Viewport},
};

const FPS: f32 = 60.0;
const FOV: f64 = 95.0;

fn main() {
    let mut view = View::new(82, 32, ColChar::EMPTY);
    let viewport = Viewport::new(
        Transform3D::new_tr(Vec3D::new(0.0, 0.0, 20.0), Vec3D::ZERO),
        FOV,
        view.center(),
    );

    let lights = vec![
        Light::new_ambient(0.3),
        Light::new_directional(0.7, Vec3D::new(1.0, -1.0, -1.0)),
    ];

    let mut donut = Mesh3D::torus(1.8, 1.0, 32, 16);

    fps_gameloop!(
        {
            donut.transform.rotation.x += 0.05;
            donut.transform.rotation.z += 0.05;
        },
        {
            view.clear();
            view.draw(
                &viewport.render(
                    vec![&donut],
                    DisplayMode::Illuminated {
                        lights: lights.clone(),
                    },
                ),
            );
            let _ = view.display_render();
        },
        FPS
    );
}
