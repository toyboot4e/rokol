/*!
Glue code for using `sokol_gfx.h` on each platform
*/

#[cfg(feature = "impl-app")]
pub mod sapp;

#[cfg(feature = "sdl2")]
pub mod sdl;
