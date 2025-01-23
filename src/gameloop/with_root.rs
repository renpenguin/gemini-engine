use std::time::{Duration, Instant};

/// This is an alternative way to handle the gameloop, separate from [`fps_gameloop!`](crate::fps_gameloop). It takes on a more object-oriented approach - here, everything related to the game is stored inside a single struct which implements [`MainLoopRoot`].
///
/// Check out the `game-loop-root.rs` example, a version of `quick-start.rs` (the example from the [core](crate::core) doc page) rewritten using `MainLoopRoot`. While in that particular case it might appear to have a lot of boilerplate code, it can make your game much easier to manage as you add more things and scale it
pub trait MainLoopRoot {
    /// This type will be passed from [`MainLoopRoot::sleep_and_get_input_data()`] to [`MainLoopRoot::frame()`]
    type InputDataType;

    /// Return the FPS at which the main loop should run. A constant like `60.0` or `30.0` is sufficient
    fn get_fps(&self) -> f32;

    /// This is where the main logic of your game should go - handling input, moving objects, handling collisions, etc.
    fn frame(&mut self, input_data: Option<Self::InputDataType>);

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

    /// The function used to sleep for the appropriate amount based on the value returned by `get_fps`. Uses [`gameloop::sleep_fps`](super::sleep_fps()) by default and will return None for the `InputDataType`. The returned bool value should represent whether or not to skip rendering on the next frame
    fn sleep_and_get_input_data(
        &self,
        fps: f32,
        elapsed: Duration,
    ) -> (bool, Option<Self::InputDataType>) {
        (super::sleep_fps(fps, Some(elapsed)), None)
    }

    /// The main loop function of the main loop root. This shouldnt be overriden. The `fps` parameter will be passed straight to [`sleep_and_get_input()`](MainLoopRoot::sleep_and_get_input_data()). See the [`MainLoopRoot`] documentation for more info
    /// ```no_run
    /// # use gemini_engine::gameloop::MainLoopRoot;
    /// # struct Game {}
    /// # impl Game {
    /// #   fn new() -> Game { Game {} }
    /// # }
    /// impl MainLoopRoot for Game {
    ///     type InputDataType = ();
    ///     fn get_fps(&self) -> f32 { 30.0 }
    ///     fn frame(&mut self, input_data: Option<Self::InputDataType>) {
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
            let (frame_skip, input_data) = self.sleep_and_get_input_data(self.get_fps(), elapsed);
            let now = Instant::now();

            self.frame(input_data);

            if !frame_skip {
                self.render_frame();
            }

            elapsed = now.elapsed();
        }
    }
}
