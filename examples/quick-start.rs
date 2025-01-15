//! The quick start example
use gemini_engine::{
    core::{ColChar, Vec2D},
    gameloop,
    primitives::Pixel,
    view::{View, WrappingMode},
};

const FPS: f32 = 30.0;

fn main() {
    let mut view = View::new(40, 8, ColChar::BACKGROUND).with_wrapping_mode(WrappingMode::Wrap);
    let mut pixel = Pixel::new(Vec2D { x: 10, y: 5 }, ColChar::SOLID);

    loop {
        view.clear();

        pixel.pos.x += 1;

        view.draw(&pixel);
        let _ = view.display_render();

        let _ = gameloop::sleep_fps(FPS, None);
    }
}
