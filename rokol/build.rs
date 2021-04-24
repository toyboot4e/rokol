use std::env;

fn main() {
    // catch which graphics backend was selected in `rokol_ffi/build.rs`
    let gfx = env::var("DEP_SOKOL_GFX").expect("`rokol_ffi` failed to select graphics backend?");
    // For `DEP_<LIB>_<VAR>`, see:
    // https://doc.rust-lang.org/cargo/reference/build-scripts.html#the-links-manifest-key

    // and emit it (glcore33, metal or d3d11):
    println!("cargo:rustc-cfg=rokol_gfx={}", gfx);
}
