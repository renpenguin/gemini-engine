use std::{thread::sleep, time::Duration};

/// Sleep for a single frame at the declared FPS, subtracting the input `Duration` to account for any time spent processing the frame. Returns a `bool` indicating whether the frame took longer to render than the intended fps wait (i.e. if there was an FPS drop)
/// ## Example
/// ```no_run
/// use gemini_engine::gameloop;
/// use std::time::Instant;
///
/// let mut frame_skip = false;
/// const FPS: f32 = 60.0;
/// loop {
///     let now = Instant::now();
///
///     // all code here will run at 60 FPS
///
///     if frame_skip {
///         frame_skip = false;
///     } else {
///         // calculations and rendering
///     }
///
///     frame_skip = gameloop::sleep_fps(FPS, Some(now.elapsed()));
/// }
#[must_use]
pub fn sleep_fps(fps: f32, elapsed: Option<Duration>) -> bool {
    let elapsed = elapsed.unwrap_or(Duration::ZERO);
    let frame_length = Duration::from_secs_f32(1.0 / fps);
    if frame_length > elapsed {
        sleep(frame_length - elapsed);
        false
    } else {
        true
    }
}
