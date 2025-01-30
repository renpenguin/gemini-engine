use std::time::{Duration, Instant};

/// This is an alternative way to handle the gameloop, separate from [`fps_gameloop!`](crate::fps_gameloop). It takes on a more object-oriented approach - here, everything related to the game is stored inside a single struct which implements [`MainLoopRoot`].
///
/// Check out the `game-loop-root.rs` example, a version of `quick-start.rs` rewritten using `MainLoopRoot`
pub trait MainLoopRoot {
    /// Return the FPS at which the main loop should run. A constant like `60.0` or `30.0` is sufficient
    fn get_fps(&self) -> f32;

    /// This is where the main logic of your game should go - handling input, moving objects, handling collisions, etc.
    fn frame(&mut self);

    /// All rendering code (drawing, printing to the screen, etc.) should be called in here. If the bool value returned by [`MainLoopRoot::sleep_and_get_input_data()`] is true, this won't run and nothing should be printed to the screen
    /// ## Example
    /// Here's an example of what a `render_frame` trait implementation might look like, assuming your root struct has a `view: View` property for your main view
    /// ```
    /// # use gemini_engine::{core::{CanDraw, Canvas}, view::View};
    /// # struct Dummy {}
    /// # impl CanDraw for Dummy {
    /// #   fn draw_to(&self, canvas: &mut impl Canvas) {}
    /// # }
    /// # struct Game {
    /// #   view: View,
    /// #   player: Dummy,
    /// #   enemies: Vec<Dummy>,
    /// # }
    /// # impl Game {
    /// // --inside impl MainLoopRoot for Game--
    /// fn render_frame(&mut self) {
    ///     self.view.clear();
    ///
    ///     // Draw every enemy in a vector of `Enemies` (all of which would implement `CanDraw`)
    ///     for enemy in &self.enemies {
    ///         self.view.draw(enemy);
    ///     }
    ///     self.view.draw(&self.player);
    ///
    ///     self.view.display_render().unwrap();
    /// }
    /// # }
    /// ```
    fn render_frame(&mut self);

    /// The function used to sleep for the appropriate amount based on the value returned by `get_fps`. Uses [`gameloop::sleep_fps`](super::sleep_fps()) by default and will return None for the `InputDataType`. If the return value is `true`, `render_frame` will not be called on the next frame
    fn sleep_and_get_input_data(&self, fps: f32, elapsed: Duration) -> bool {
        super::sleep_fps(fps, Some(elapsed))
    }

    /// The main loop function of the main loop root. This shouldn't be overriden. See the [`MainLoopRoot`] documentation for more info
    /// ```no_run
    /// # use gemini_engine::gameloop::MainLoopRoot;
    /// # struct Game {}
    /// # impl Game {
    /// #   fn new() -> Game { Game {} }
    /// # }
    /// impl MainLoopRoot for Game {
    ///     fn get_fps(&self) -> f32 { 30.0 }
    ///     fn frame(&mut self) {
    ///         // --snip--
    ///     }
    ///     fn render_frame(&mut self) {
    ///         // --snip--
    ///     }
    /// }
    /// let mut game = Game::new(); // `Game` implements `MainLoopRoot`. Its `new` method sets up all the game objects
    ///
    /// game.main_loop();
    /// ```
    fn main_loop(&mut self) {
        let mut elapsed = Duration::ZERO;

        loop {
            let frame_skip = self.sleep_and_get_input_data(self.get_fps(), elapsed);
            let now = Instant::now();

            self.frame();

            if !frame_skip {
                self.render_frame();
            }

            elapsed = now.elapsed();
        }
    }
}
