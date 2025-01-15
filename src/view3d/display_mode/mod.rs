pub mod lighting;
use lighting::Light;

/// `DisplayMode` determines how the [`Viewport`](super::Viewport) renders our 3D objects. This is the Gemini equivalent of Blender's Viewport Shading options
#[derive(Debug, Clone, PartialEq)]
pub enum DisplayMode {
    /// `DisplayMode::Wireframe` renders the edges of the meshes, without filling in the shapes. You can choose whether you want to render with backface culling using the [`backface_culling`](DisplayMode::Wireframe::backface_culling) enum parameter
    Wireframe {
        /// Whether or not to enable backface culling (parts of the mesh with faces that are not facing towards the viewport will be removed)
        backface_culling: bool,
    },
    /// `DisplayMode::Solid` renders the full faces of all the meshes.
    Solid,
    /// `DisplayMode::Illuminated` will replace your faces' `text_char`s with a character of different size to emulate light, based on a given list of [`Light`]s
    Illuminated {
        /// The collection of lights used to illuminate the scene
        lights: Vec<Light>,
    },
}
