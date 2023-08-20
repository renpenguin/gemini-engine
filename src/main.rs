use std::time::Instant;
mod elements;
use elements::{Box, Line, Point, Sprite, Triangle, Vec2D, View};
mod gameloop;

const FPS: u32 = 20;
const FILL_CHAR: char = '█';
const BACKGROUND_CHAR: char = '░';

fn main() {
    let mut view = View::new(60, 10, BACKGROUND_CHAR);

    let mut point_pos = Vec2D::ZERO;
    point_pos += Vec2D::from((5, 9));
    let mut line1_direction = -1;

    let point1 = Point::new(point_pos + Vec2D::new(2, -8), FILL_CHAR);

    let mut line1 = Line::new(Vec2D::new(2, 8), Vec2D::new(28, 7), FILL_CHAR);

    let box1 = Box::new(Vec2D { x: 11, y: 1 }, Vec2D { x: 9, y: 3 }, FILL_CHAR);

    let triangle1 = Triangle::new(
        Vec2D::new(32, 1),
        Vec2D::new(54, 3),
        Vec2D::new(40, 5),
        FILL_CHAR,
    );

    let test_image = r"
  ______
 /|_||_\`.__
(   _    _ _\
=`-(_)--(_)-'   ";
    let mut sprite1 = Sprite::new(Vec2D::new(10, 1), test_image);

    loop {
        // Begin game loop
        let now = Instant::now();
        view.clear();

        point_pos.x += 1;
        point_pos %= Vec2D::from(&view); // loop the position back to the other side

        line1.pos1.y += line1_direction;
        line1.pos0.y = 10 - line1.pos1.y;
        if line1.pos1.y > 7 {
            line1_direction = -1;
        } else if line1.pos1.y < 3 {
            line1_direction = 1;
        }
        line1.generate_cache();

        sprite1.pos.x += 1;
        sprite1.pos %= Vec2D::from(&view);

        view.plot(point_pos, FILL_CHAR);

        view.blit(&point1);
        view.blit(&line1);
        view.blit(&box1);
        view.blit(&sprite1);
        view.blit(&triangle1);

        view.render();
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?} microseconds", elapsed.as_micros());
        println!("Point position: {point_pos}");

        gameloop::sleep_fps(FPS, Some(elapsed)); // not making use of frame_skip as this particular View is very simple and unlikely to exceed frame duraction
    }
}
