//! This modules holds some miscellaneous tools for containing and manipulating `CanDraw` elements. Since every container itself implements [`Candraw`](crate::core::CanDraw), containers can often be combined by nesting inside of each other.

mod visibility_toggle;
pub use visibility_toggle::VisibilityToggle;

mod pixel_container;
pub use pixel_container::PixelContainer;

mod shader;
pub use shader::CanShade;

mod collision_container;
pub use collision_container::{CanCollide, CollisionContainer};
