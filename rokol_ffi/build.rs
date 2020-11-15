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

use std::{env, path::PathBuf};

use cc::Build;

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

    // pub fn set_bindgen_flag(&self, b: bindgen::Builder) -> bindgen::Builder {
    //     match self {
    //         Self::D3D11 => b.clang_arg("-DSOKOL_D3D11"),
    //         Self::Metal => b.clang_arg("-DSOKOL_METAL"),
    //         Self::GlCore33 => b.clang_arg("-DSOKOL_GLCORE33"),
    //     }
    // }

    pub fn set_cflag(&self, build: &mut Build) {
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
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let sokol_dir = root.join("sokol");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // ----------------------------------------
    // Get metadata

    let mut build = Build::new();
    let tool = build.try_get_compiler().unwrap();
    let is_msvc = tool.is_like_msvc();
    let will_set_debug_flags = env::var("DEBUG").ok().is_some();

    let renderer = Renderer::get(is_msvc);

    // ----------------------------------------
    // Generate FFI

    // We have to use `.n` extension in macOS even if the file contents are the same
    let wrapper = if cfg!(target_os = "macos") {
        PathBuf::from(wrapper).with_extension("m")
    } else {
        PathBuf::from(wrapper)
    };
    let wrapper = format!("{}", wrapper.display());

    let bindings = {
        let b = bindgen::builder();
        let b = b.header(&wrapper);
        let b = b.clang_arg(format!("-I{}", sokol_dir.display()));
        // let b = renderer.set_bindgen_flag(b);
        b.generate().unwrap()
    };

    bindings
        .write_to_file(out_dir.join(ffi_file))
        .expect("Couldn't write bindings!");

    // ----------------------------------------
    // Set up compiler flags

    build.include(format!("{}/sokol", root.display()));

    // MacOS: need ARC
    if cfg!(target_os = "macos") {
        build.flag("-fobjc-arc");
    }

    // TODO: order?
    build.file(&wrapper);

    renderer.set_cflag(&mut build);
    renderer.link();

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

    // TODO: enable overriding sokol debug flags
    if will_set_debug_flags {
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
        match renderer {
            Renderer::Metal => {
                println!("cargo:rustc-link-lib=framework=Metal");
                println!("cargo:rustc-link-lib=framework=MetalKit");
            }
            Renderer::GlCore33 => {
                todo!();
            }
            Renderer::D3D11 => panic!("Trying to use D3D11 on macOS"),
        }
        println!("cargo:rustc-link-lib=framework=AudioToolbox");
    }

    // Linux: libs
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=dylib=GL");
        println!("cargo:rustc-link-lib=dylib=X11");
        println!("cargo:rustc-link-lib=dylib=asound");
    }
}
