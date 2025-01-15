//! An example of a spinning cube with `elements3d`
use gemini_engine::{
    core::ColChar,
    fps_gameloop, gameloop,
    mesh3d::Mesh3D,
    view::View,
    view3d::{DisplayMode, Light, Transform3D, Vec3D, Viewport},
};

const FPS: f32 = 30.0;
const FOV: f64 = 95.0;

fn main() {
    let mut view = View::new(200, 90, ColChar::BACKGROUND);

    let viewport = Viewport::new(
        Transform3D::new_tr(Vec3D::new(0.0, 1.5, 4.0), Vec3D::new(-0.4, 0.0, 0.0)),
        FOV,
        view.center(),
    );

    let mut cube = Mesh3D::default_cube();

    fps_gameloop!(
        {
            view.clear();
            cube.transform.rotation.y -= 0.05;
        },
        {
            view.draw(&viewport.render(vec![&cube], DisplayMode::Solid));
            let _ = view.display_render();
        },
        FPS,
        |elapsed: gameloop::Duration, frame_skip| {
            println!(
                "Elapsed: {:.2?}µs | Frame skip: {}",
                elapsed.as_micros(),
                frame_skip
            );
        }
    );
}
