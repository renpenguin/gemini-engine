#![doc = include_str!("../README.md")]
//!
//! ## Crate Structure
//! This library is made up of several modules:
//! - [`core`] declares the relationship between any [`Canvas`](core::Canvas) (an object that can be drawn to) and the library's primitives and anything else that can be drawn to the screen.
//! - [`view`] defines the [`View`](view::View), a [`Canvas`](core::Canvas) capable of displaying
//! - [`ascii`], [`containers`] and [`primitives`] which all offer different ways to draw to a [`Canvas`](core::Canvas). [`containers`] also has a basic collision library!
//! - [`mesh3d`] and [`view3d`], which hold the [`Mesh3d`](mesh3d::Mesh3D) and [`Viewport`](view3d::Viewport) objects respectively, and handle everything 3D-related. [`Viewport`](view3d::Viewport)

pub mod ascii;
pub mod containers;
pub mod core;
pub mod primitives;
pub mod view;

#[cfg(feature = "gameloop")]
pub use gemini_mainloop as gameloop;
#[cfg(feature = "gameloop")]
pub use gemini_mainloop::fps_gameloop;

#[cfg(feature = "3D")]
pub mod mesh3d;
#[cfg(feature = "3D")]
pub mod view3d;
