//! `quick-start.rs` recreated using [`gameloop::MainLoopRoot`](gemini_engine::gameloop::MainLoopRoot)

use gemini_engine::gameloop::MainLoopRoot;
use gemini_engine::{
    core::{ColChar, Vec2D},
    primitives::Pixel,
    view::{View, WrappingMode},
};

struct Game {
    view: View,
    pixel: Pixel,
}

impl Game {
    fn new() -> Self {
        Self {
            view: View::new(40, 8, ColChar::BACKGROUND).with_wrapping_mode(WrappingMode::Wrap),
            pixel: Pixel::new(Vec2D { x: 10, y: 5 }, ColChar::SOLID),
        }
    }
}

impl MainLoopRoot for Game {
    fn get_fps(&self) -> f32 {
        30.0
    }

    fn frame(&mut self) {
        self.pixel.pos.x += 1;
    }

    fn render_frame(&mut self) {
        self.view.clear();
        self.view.draw(&self.pixel);
        let _ = self.view.display_render();
    }
}

fn main() {
    let mut game = Game::new();

    game.main_loop();
}
