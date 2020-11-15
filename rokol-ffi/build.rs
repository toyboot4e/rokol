//! Build script of `rokol-ffi`
//!
//! # Speifying backend
//!
//! You can select Sokol renderer from `build.rs` in a parent directory:
//!
//! ```no_run
//! println!("cargo:rustc-env=ROKOL_RENDERER=<renderer_of_your_preference>");
//! ```
//!
//! Or a default renderer will be chosen.

use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

use cc::{Build, Tool};

fn main() {
    self::make_sokol("wrappers/app.h", "sokol_app_ffi.rs");
}

enum Renderer {
    D3D11,
    Metal,
    GlCore33,
}

impl Renderer {
    pub fn get(is_msvc: bool) -> Self {
        if let Ok(rdr) = env::var("ROKOL_RENDERER") {
            match rdr.as_str() {
                "D3D11" => Renderer::D3D11,
                "METAL" => Renderer::Metal,
                "GlCore33" => Renderer::GlCore33,
                _ => panic!("ROKOL_RENDERER is invalid: {}", rdr),
            }
        } else {
            // - Windows: D3D11 with MSVC, GLCORE33 otherwise
            // - MacOS: Metal
            // - Linux: GLCORE33
            if cfg!(target_os = "windows") && is_msvc {
                Self::D3D11
            } else if cfg!(target_os = "macos") {
                Self::Metal
            } else {
                Self::GlCore33
            }
        }
    }

    pub fn set_flag(&self, build: &mut Build) {
        match self {
            Self::D3D11 => build.flag("-DSOKOL_D3D11"),
            Self::Metal => build.flag("-DSOKOL_METAL"),
            Self::GlCore33 => build.flag("-DSOKOL_GLCORE33"),
        };
    }

    pub fn link(&self) {
        match self {
            Self::D3D11 => println!("cargo:rustc-cfg=gfx=\"d3d11\""),
            Self::Metal => println!("cargo:rustc-cfg=gfx=\"metal\""),
            Self::GlCore33 => println!("cargo:rustc-cfg=gfx=\"glcore33\""),
        }
    }
}

/// Compiles the given `wrapper` file and create FFI to it
fn make_sokol(wrapper: &str, ffi_file: &str) {
    let mut build = Build::new();
    let tool = build.try_get_compiler().unwrap();

    let is_debug = env::var("DEBUG").ok().is_some();
    let is_msvc = tool.is_like_msvc();

    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let sokol_dir = root.join("sokol");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // ----------------------------------------
    // Generate FFI

    let bindings = bindgen::builder()
        // .header(format!("{}", root.join(wrapper).display()))
        .header(wrapper)
        .clang_arg(format!("-I{}", sokol_dir.display()))
        .generate()
        .unwrap();

    bindings
        .write_to_file(out_dir.join(ffi_file))
        .expect("Couldn't write bindings!");

    return;

    // ----------------------------------------
    // Setup compiler flags

    build.include(format!("{}/sokol", root.display()));

    // MacOS: need ARC
    if cfg!(target_os = "macos") {
        build.flag("-fobjc-arc");
    }

    // TODO: order?
    build.file(wrapper);

    {
        let rdr = Renderer::get(is_msvc);
        rdr.set_flag(&mut build);
        rdr.link();
    }

    // x86_64-pc-windows-gnu: additional compile/link flags
    if cfg!(target_os = "windows") && !is_msvc {
        build
            .flag("-D_WIN32_WINNT=0x0601")
            .flag_if_supported("-Wno-cast-function-type")
            .flag_if_supported("-Wno-sign-compare")
            .flag_if_supported("-Wno-unknown-pragmas");

        println!("cargo:rustc-link-lib=static=gdi32");
        println!("cargo:rustc-link-lib=static=ole32");
    }

    if is_debug {
        // TODO: is this correct: `_DEBUG`
        build.flag("-D_DEBUG").flag("-DSOKOL_DEBUG");
    }

    // ----------------------------------------
    // Compile

    build.flag("-DSOKOL_IMPL");
    build.compile("sokol");

    // ----------------------------------------
    // Platform dependent libraries

    // MacOS: frameworks
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=Cocoa");
        println!("cargo:rustc-link-lib=framework=QuartzCore");
        println!("cargo:rustc-link-lib=framework=Metal");
        println!("cargo:rustc-link-lib=framework=MetalKit");
        println!("cargo:rustc-link-lib=framework=AudioToolbox");
    }

    // Linux: libs
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=dylib=GL");
        println!("cargo:rustc-link-lib=dylib=X11");
        println!("cargo:rustc-link-lib=dylib=asound");
    }
}
