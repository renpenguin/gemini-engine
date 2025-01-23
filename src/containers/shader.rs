use crate::{containers::PixelContainer, primitives::Pixel};

/// To write a shader you must have a struct that implements this shader
pub trait CanShade {
    /// This function accepts a pixel and returns the adjusted pixel, as you wish to adjust it
    fn shade(&mut self, pixel: Pixel) -> Pixel;
}

impl PixelContainer {
    /// Applies the shader to the `PixelContainer`'s active pixels. A "shader" in this case is any object which implements [`CanShade`]
    #[must_use]
    pub fn shade_with(mut self, shader: &mut Box<dyn CanShade>) -> Self {
        self.pixels
            .iter_mut()
            .for_each(|p| *p = shader.shade(*p));
        self
    }
}
