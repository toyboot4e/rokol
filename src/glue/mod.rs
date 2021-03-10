/*!
Glue code for each application backend
*/

#[cfg(feature = "use-sokol-app")]
pub mod sapp;

#[cfg(feature = "use-sdl2")]
pub mod sdl;
