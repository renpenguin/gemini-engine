//! An example of a self-resizing view using `ScaleFitView`
use gemini_engine::{
    ascii::{Sprite, Text, TextAlign, TextAlign2D},
    core::{ColChar, Modifier, Vec2D},
    view::ScaleFitView,
};
use std::{thread, time::Duration};

const TEXTURE: &str = "
.-----.
|     |
| hi! |
|     |
`-----'";

fn main() {
    let mut scale_view = ScaleFitView::new(ColChar::BACKGROUND);

    let mut text = Text::new(Vec2D::ZERO, "This is some centered text!", Modifier::None)
        .with_align(TextAlign::Centered);

    let mut sprite =
        Sprite::new(Vec2D::ZERO, TEXTURE, Modifier::None).with_align(TextAlign2D::CENTERED);

    loop {
        text.pos = scale_view.intended_size() / 2;
        sprite.pos = scale_view.intended_size() / 2;
        sprite.pos.y -= 5;

        scale_view.update();
        scale_view.view.draw(&text);
        scale_view.view.draw(&sprite);
        let _ = scale_view.view.display_render();

        thread::sleep(Duration::from_millis(10));
    }
}
