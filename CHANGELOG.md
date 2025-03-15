# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.2.0] - 2025-03-15

### Changed

- Implemented a much faster way to draw at double width!
- Moved the gameloop module to its own crate: [gemini-mainloop](https://crates.io/crates/gemini-mainloop). This has no effective change

### Breaking changes

- `MainLoopRoot`'s (now in `gemini-mainloop`) `sleep_and_get_input_data` method has been renamed to just `sleep`

### Fixed

- Integer underflow error when trying to call `View::display_render` in a binary built on debug

## [1.1.1] - 2025-02-01

### Changed

- Nicer error message when face index is out of bounds on passed slice in `Face::index_into`
- Don't sleep before calling the first `frame` and `render_frame` in `MainLoopRoot::main_loop`

## [1.1.0] - 2025-01-27

### Breaking changes

- Removed input handling from `MainLoopRoot`. Input handling should be done entirely within the `input` method

## [1.0.1] - 2025-01-24

### Added

- Implented `CanCollide` for `Pixel` and `Rect`. Remember that you can still generate a collider for any object by drawing it into a `PixelContainer`, as that implements `CanCollide`!
- This CHANGELOG file

### Fixed

- `AnimatedSprite::set_current_frame` no longer performs a bitwise AND where it should be performing a `rem_euclid` operation

### Changed

- Bumped `glam` version to `v0.28.0`. Currently any higher leads to a build failure do to some failure with the external library
- `gameloop` is now an on-by-default feature

## [1.0.0] - 2025-01-24

I came back to `gemini-engine` with the intent to restructure it from the ground up and to make use of `glam` for vectors and matrix transformations, with zero mercy towards the old codebase, so a LOT has been changed. You can use the list of breaking changes below to find your way around migrating your old gemini project to 1.0.

### Breaking changes

- The entire library has been restructured:
	- `elements` has been split into `core`, `view`, `ascii`, `primitives` and `containers`. 
		`core` contains the main traits (`CanDraw` and `Canvas`) and their relevant types `Vec2D`, `ColChar`, `Modifier` and `Colour`
	- `elements3d` has been split into `mesh3d` (where `Mesh3D`, `Transform3D`, `Vec3D` and `Face` now live) 
		and `view3d` (for `Viewport`, `DisplayMode` and related types)
	- `gameloop` remains much the same, but `with_root` has been flattened into `gameloop` so that `MainLoopRoot` is now accessible directly from the `gameloop` module
		- The FPS at which a `MainLoopRoot` runs is now determined by 

- Replaced vectors and transforms with `glam` primitives:
	- `Vec2D` is now an alias to `glam::I64Vec2`, replacing most instances of `isize` with `i64`
	- `Vec3D` is now an alias to `glam::DVec3`
	- `Transform3D` has been replaced with an alias to `glam::DMat4`. 
		This completely changes how transformation of 3D objects is done. See examples for common uses
	- Certain methods are now named differently.
		For example, the vectors' `cross` method is now called `perp_dot`

- Replaced `ViewElement` with `CanDraw` and `CanCollide`
	- `CanDraw` takes a mutable reference to a `Canvas` implementing object (such as `View`) and draws to it using the `plot`
	- All primitives have been reworked to use `CanDraw::draw_to` instead of `ViewElement::active_pixels`
	- `CanCollide` is only required for `CollisionContainer`, but is to be implemented by many common primitives to allow more efficient collisions
	- `PixelContainer` now acts as an in-between plane -
		objects can be drawn to the container, and the container can in turn draw to any other `Canvas`

- `Viewport` has been restructured with the above changes in mind:
	- `Viewport::render` has been removed. It has been replaced with an implementation of `CanDraw` which draws faces directly to a passed_screen. 
	- The list of models and `display_mode` are now fields of the struct, to be set in advance

- All instances of the keyword `blit` and often `print` have been replaced with `draw`
- `Rect::new_from_to` offsets size by `Vec2D::ONE` to match the visual appearance of a Rect 
	(previously, using `new_from_to` with the same position twice would give a `Rect` with size `(0,0)`)

### Added

- `Rect::bottom_right` returns the position of a `Rect` object's bottom right pixel

### Removed

- `ascii::remove_leading_newlines`, in favour of `txt.trim_start_matches('\n')`
- `gameloop::{Duration, Instant}`, in favour of `std::time::{Duration, Instant}`
- `impl FromStr for Vec3D`
- Individual primitive draw functions, such as `Line::draw()`
- `DisplayMode`'s `Debug` and `Points` variants.

### Changed

- A full rewrite of the documentation to match all the changes
- Major performance improvements to the 3D engine!
- `Wrapping` has been renamed to `WrappingMode` and is now defined for the entire `View`, instead of being set per-

## [0.14.2] - 2024-05-02

### Changed

- Replaced `termsize` with `terminal_size`, as `termsize` was broken
- Implemented defaults for containers
- Nicer errors and docs

[unreleased]: https://github.com/renpenguin/gemini-engine/compare/v1.2.0...HEAD
[1.2.0]: https://github.com/renpenguin/gemini-engine/compare/v1.1.1...1.2.0
[1.1.1]: https://github.com/renpenguin/gemini-engine/compare/v1.1.0...v1.1.1
[1.1.0]: https://github.com/renpenguin/gemini-engine/compare/v1.0.1...v1.1.0
[1.0.1]: https://github.com/renpenguin/gemini-engine/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/renpenguin/gemini-engine/compare/v0.14.2...v1.0.0
[0.14.2]: https://github.com/renpenguin/gemini-engine/compare/v0.14.1...v0.14.2