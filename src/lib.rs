/*!
Wrapper of [Sokol] libraries (`sokol_gfx.h` and `sokol_app.h`)

Check out the [examples] and [Learn OpenGL examples] (WIP) to get more information. You can switch
features to use Rust-SDL2 platform.

[Sokol]: https://github.com/floooh/sokol
[examples]: https://github.com/toyboot4e/rokol/tree/master/examples
[Learn OpenGL examples]: https://github.com/toyboot4e/rokol_learn_opengl

TIP: Sokol [considers] zero-initizialized structures to be in default state. It means
[`Default::default`] is ensured to make sense!

[considers]: https://floooh.github.io/2017/08/06/sokol-api-update.html

# Status

Early in progress: **currently only macOS + GlCore33 backend is considered**

I'd do Learn OpenGL examples to make it better.
*/

pub use rokol_ffi as ffi;

#[cfg(feature = "impl-app")]
pub mod app;

#[cfg(feature = "impl-gfx")]
pub mod gfx;

#[cfg(feature = "impl-gfx")]
pub mod glue;

#[cfg(all(feature = "impl-gfx", feature = "use-fontstash"))]
pub mod fons;
