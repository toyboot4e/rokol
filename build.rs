//! If you need to do conditional compilation with Rokol renderer, add `rokol_ffi` to your
//! `Cargo.toml` and then copy this build script to your project.
//!
//! Then you can use `#[cfg(rokol_gfx = "d3d11")]` ("d3d11" may be "metal" or "glcore33").

use std::env;

fn main() {
    println!("cargo:rerun-if-env-changed=ROKOL_RENDERER");

    // Catch `DEP_SOKOL_GFX` defined in `sokol_ffi/build.rs`
    let gfx = env::var("DEP_SOKOL_GFX").expect("`rokol_ffi` failed to select graphics backend?");
    // Enable conditional compilation with `rokol_gfx` in this crate
    // (`rokol_gfx` is one of `d3d11`, `metal` or `glcore33`)
    println!("cargo:rustc-cfg=rokol_gfx={}", gfx);
}

// Note that `rokol_ffi` can emit `DEP_SOKOL_GFX` to only those crates that have `rokol_ffi` in
// their `Cargo.toml`!
