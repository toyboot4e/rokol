# rokol-ffi

Rust FFI to [Sokol](https://github.com/floooh/sokol), only for [Rokol](https://github.com/toyboot4e/rokol)

It's generated with [bindgen](https://github.com/rust-lang/rust-bindgen) and implements `Default` trait.

## Status

**Tested on macOS only**

## Supported headers

`rokol_ffi` compiles all of the follows:

* `sokol_app.h`
* `sokol_gfx.h`
* `sokol_glue.h`

## Supported backends

GlCore33, Metal and D3D9. **WebGPU backend is not supported by rokol-ffi** (until I need it).

## Specifying renderer

To specify the renderer, use feature flag or set `ROKOL_RENDERER`. Changing `ROKOL_RENDERER` results in recompilation, so I'd recommend not changing it frequently (if possible).

## Conditional compilation in down stream crates

Emits `DEP_SOKOL_GFX` to `build.rs` of crates that lists `rokol_ffi` in their `Cargo.toml`. T if you're interestedhat can be used for conditional compilation.

c.f. [Build Scripts - The Cargo Book #The links Manifest Key](https://doc.rust-lang.org/cargo/reference/build-scripts.html#the-links-manifest-key)

