/*!

Wrapper of [Sokol] libraries

Check out the [examples] to get more information.

[Sokol]: https://github.com/floooh/sokol
[examples]: https://github.com/toyboot4e/rokol/blob/master/examples

NOTE: Sokol [considers] zero-initizialized structures to be in default state. It means
[`Default::default`] is ensured to make sense!

[considers]: https://floooh.github.io/2017/08/06/sokol-api-update.html

# Status

This crate is very early in progress. I'd do Learn OpenGL examples to make it better.

TODO: ImGUI support

*/

pub use rokol_ffi as ffi;

#[cfg(feature = "use-sokol-app")]
pub mod app;

pub mod gfx;

// #[cfg(feature = "debug-ui")]
// pub mod imgui;

#[cfg(feature = "use-fontstash")]
pub mod fons;
