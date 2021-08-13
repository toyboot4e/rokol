/*!
Wrapper of [Sokol] libraries

[Sokol]: https://github.com/floooh/sokol

NOTE: `rokol` is still early in progress: **currently only macOS + GlCore33 backend is considered**

# Features (specified in `Cargo.toml`)

* `impl-app`: imports `sokol_app.h` and enables [`app`] module
* `sdl2`: generates [`glue`] code for `sdl2`
* `impl-gfx`: imports `sokol_gfx.h` and enables [`gfx`] module
  * `glcore33`: use OpenGL backend
  * `metal`: use Metal backend
  * `d3d11`: use DirectX11 backend
* `fontstash`: imports `fontstash.h` and enables [`fons`] module

# Tips

* Checkout [The Brain Dump]
  * Sokol [considers] zero-initizialized structures to be in default state. It means
    [`Default::default`] is ensured to make sense!
* use `bytemuck` to cast types to `&[u8]`.

[The Brain Dump]: https://floooh.github.io/
[considers]: https://floooh.github.io/2017/08/06/sokol-api-update.html
*/

pub use rokol_ffi as ffi;

#[cfg(feature = "impl-app")]
pub mod app;

#[cfg(feature = "impl-gfx")]
pub mod gfx;

#[cfg(feature = "impl-gfx")]
pub mod glue;

#[cfg(all(feature = "impl-gfx", feature = "fontstash"))]
pub mod fons;
