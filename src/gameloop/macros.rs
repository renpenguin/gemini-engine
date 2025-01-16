/// You can use the `fps_gameloop!` macro to avoid writing a lot of boilerplate code. Take this block of code from a program written with gemini, for example:
/// ```no_run
/// # use gemini_engine::{core::{ColChar, Vec2D}, view::View, mesh3d::{Mesh3D, Transform3D}, view3d::Viewport, gameloop};
/// # use std::time::Instant;
/// # let mut view = View::new(0, 0, ColChar::BACKGROUND);
/// # let mut viewport = Viewport::new(Transform3D::default(), 0.0, Vec2D::ZERO);
/// viewport.objects.push(Mesh3D::default_cube());
/// let FPS = 30.0;
///
/// let mut frame_skip = false;
/// loop {
///     let now = Instant::now();
///
///     // Logic
///     let cube_transform = &mut viewport.objects[0].transform;
///     *cube_transform = cube_transform
///         .mul_mat4(&Transform3D::from_rotation_y(-0.05));
///
///     if frame_skip {
///         frame_skip = false;
///     } else {
///         // Rendering
///         view.clear();
///         view.draw(&viewport);
///         view.display_render().unwrap();
///     }
///     let elapsed = now.elapsed();
///     frame_skip = gameloop::sleep_fps(FPS, Some(elapsed));
/// }
/// ```
/// There's a lot of boilerplate code here. That's where this macro comes in. Here is the same block of code, rewritten with `fps_gameloop!`:
/// ```no_run
/// # use gemini_engine::{core::{ColChar, Vec2D}, view::View, view3d::{Viewport, DisplayMode}, mesh3d::{Mesh3D, Transform3D}, fps_gameloop};
/// # let mut view = View::new(0, 0, ColChar::BACKGROUND);
/// # let mut viewport = Viewport::new(Transform3D::default(), 0.0, Vec2D::ZERO);
/// viewport.objects.push(Mesh3D::default_cube());
/// let FPS = 30.0;
///
/// fps_gameloop!(
///     {
///         let cube_transform = &mut viewport.objects[0].transform;
///         *cube_transform = cube_transform
///             .mul_mat4(&Transform3D::from_rotation_y(-0.05));
///     },
///     {
///         view.clear();
///         view.draw(&viewport);
///         view.display_render().unwrap();
///     },
///     FPS
/// );
/// ```
/// The code is now a lot less cluttered. This macro accepts three fragments (and an optional fourth fragment):
/// - A logic block fragment for code that should run every single frame
/// - A render block fragment for code related to displaying to the terminal (all plots, draws and renders). This will not run if the previous frame took too long
/// - An `f32` fragment representing the desired frames per second.
/// - Optionally, a function of type `Fn(`[`Duration`](std::time::Duration)`, bool)`. The passed duration will be the time taken to render everything, and the passed `bool` indicates whether the last frame was skipped or not. It can be used to, say, print debug info. Here's an example:
/// ```no_run
/// # use gemini_engine::{fps_gameloop, gameloop};
/// # use std::time::Duration;
/// fps_gameloop!(
///     // -- other fields --
/// #   {}, {}, 0.0,
///     |elapsed: Duration, frame_skip: bool| {
///         println!(
///             "Elapsed: {:.2?}µs | Frame skip: {}",
///             elapsed.as_micros(),
///             frame_skip
///         );
///     }
/// );
#[macro_export]
macro_rules! fps_gameloop {
    ($logic:block, $render:block, $fps:expr) => {
        fps_gameloop!($logic, $render, $fps, |_, _| ());
    };
    ($logic:block, $render:block, $fps:expr, $handle_elapsed:expr) => {
        use std::time::Instant;
        let mut frame_skip = false;
        loop {
            let now = Instant::now();

            $logic; // Logic

            match frame_skip {
                true => frame_skip = false,
                false => {
                    $render; // Rendering
                }
            }

            // Debug info and such
            ($handle_elapsed)(now.elapsed(), frame_skip);

            let elapsed = now.elapsed();
            frame_skip = gemini_engine::gameloop::sleep_fps($fps, Some(elapsed));
        }
    };
}
