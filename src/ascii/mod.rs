//! This module holds the structs related to display of ASCII characters, both text and ASCII art

mod animated_sprite;
pub use animated_sprite::AnimatedSprite;

mod sprite;
pub use sprite::Sprite;

mod text;
pub use text::Text;

mod alignment;
pub use alignment::{TextAlign, TextAlign2D};
