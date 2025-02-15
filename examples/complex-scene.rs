//! An example of a more complex scene in Gemini
use gemini_engine::{
    ascii::Sprite,
    core::{ColChar, Modifier, Vec2D},
    fps_gameloop,
    primitives::{Line, Pixel, Rect},
    view::{View, WrappingMode},
};
use std::time::Duration;

const FPS: f32 = 20.0;
const FILL_CHAR: ColChar = ColChar::SOLID;
const BACKGROUND_CHAR: ColChar = ColChar::EMPTY;

fn main() {
    let mut view = View::new(60, 10, BACKGROUND_CHAR);

    let mut pixel = Pixel::new(Vec2D::new(5, 9), FILL_CHAR);

    let mut line = Line::new(Vec2D::new(2, 8), Vec2D::new(28, 7), FILL_CHAR);
    let mut line1_direction = -1;

    let rect = Rect::new(
        Vec2D { x: 11, y: 1 },
        Vec2D { x: 9, y: 3 },
        ColChar::SOLID.with_rgb(200, 30, 0),
    );

    let test_image = r"
  ______
 /|_||_\`.__
(   _    _ _\
=`-(_)--(_)-'   ";
    let mut sprite = Sprite::new(
        Vec2D::new(30, 1),
        test_image,
        Modifier::from_rgb(20, 200, 0),
    );

    let mut draw_elapsed = Duration::default();
    let mut render_elapsed = Duration::default();
    fps_gameloop!(
        {
            pixel.pos.x += 2;
            // loop the position back to the other side. This can be done with `WrappingMode::Wrap` but it won't change the element's actual position, so the pixel position being printed would continue to increase without looping
            pixel.pos %= view.size();

            line.pos1.y += line1_direction;
            line.pos0.y = 10 - line.pos1.y;
            if line.pos1.y > 7 {
                line1_direction = -1;
            } else if line.pos1.y < 3 {
                line1_direction = 1;
            }

            sprite.pos.x += 1;
        },
        {
            view.clear();

            let now = Instant::now();
            view.wrapping_mode = WrappingMode::Panic;
            view.draw(&pixel);
            view.draw(&line);
            view.draw(&rect);
            view.wrapping_mode = WrappingMode::Wrap;
            view.draw(&sprite);
            draw_elapsed = now.elapsed();

            let now = Instant::now();
            let _ = view.display_render();
            render_elapsed = now.elapsed();
        },
        FPS,
        |total_elapsed: Duration, _frame_skip| {
            println!(
                "Drawing: {:.2?} microseconds | Rendering: {:.2?} microseconds| Total: {:.2?}",
                draw_elapsed.as_micros(),
                render_elapsed.as_micros(),
                total_elapsed.as_micros()
            );
            println!("Pixel position: {}", pixel.pos);
        }
    );
}
