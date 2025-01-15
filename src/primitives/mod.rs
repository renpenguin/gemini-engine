//! This module contains basic geometry structs that implement [`ViewElement`](super::view::ViewElement), such as [`Line`] or [`Triangle`]

mod helpers;
pub use helpers::{triangulate, interpolate, interpolate_floating};

mod line;
pub use line::Line;

mod pixel;
pub use pixel::Pixel;

mod polygon;
pub use polygon::Polygon;

mod rect;
pub use rect::Rect;

mod triangle;
pub use triangle::Triangle;