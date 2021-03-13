/*!
Glue code for using `sokol_gfx.h` on each platform
*/

#[cfg(feature = "use-sokol-app")]
pub mod sapp;

#[cfg(feature = "use-sdl2")]
pub mod sdl;
