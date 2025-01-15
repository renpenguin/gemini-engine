//! An example of [`fps_gameloop!`] in action

use std::{thread, time::Duration};

use gemini_engine::{
    core::{ColChar, Vec2D},
    fps_gameloop,
    primitives::Rect,
    view::{View, WrappingMode},
};

const BLOCK_SIZE: Vec2D = Vec2D::new(4, 2);
const FILL_CHAR: ColChar = ColChar::SOLID;

fn main() {
    let mut view = View::new(50, 12, ColChar::BACKGROUND).with_wrapping_mode(WrappingMode::Wrap);

    let mut blocks = vec![
        Rect::new(Vec2D::new(0, 0), BLOCK_SIZE, FILL_CHAR),
        Rect::new(Vec2D::new(0, 2), BLOCK_SIZE, FILL_CHAR),
        Rect::new(Vec2D::new(0, 4), BLOCK_SIZE, FILL_CHAR),
        Rect::new(Vec2D::new(0, 6), BLOCK_SIZE, FILL_CHAR),
        Rect::new(Vec2D::new(0, 8), BLOCK_SIZE, FILL_CHAR),
        Rect::new(Vec2D::new(0, 10), BLOCK_SIZE, FILL_CHAR),
    ];

    let mut i = 0;
    fps_gameloop!(
        {
            i += 1;
            for (j, block) in (0u32..).zip(blocks.iter_mut()) {
                if i % 2_u32.pow(j) == 0 {
                    block.pos.x += 1;
                }
            }
        },
        {
            view.clear();
            for block in &blocks {
                view.draw(block);
            }
            let _ = view.display_render();

            if blocks.iter().all(|b| b.pos.x % view.width as i64 == 0) {
                thread::sleep(Duration::from_secs(2));
            };
        },
        60.0
    );
}
