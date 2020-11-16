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
//! Or a default renderer for each backend will be chosen.
//!
//! # Forcing debug flag for Sokol
//!
//! Set `ROKOL_FORCE_DEBUG` to any value to enable debug mode in release build:
//!
//! ```no_run
//! println!("cargo:rustc-env=ROKOL_FORCE_DEBUG=TRUE");
//! ```

use std::{
    env,
    path::{Path, PathBuf},
};

use cc::Build;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let debug = env::var("ROKOL_FORCE_DEBUG").ok().is_some() || env::var("DEBUG").ok().is_some();

    self::gen_bindings("wrappers/app.h", &out_dir.join("sokol_app_ffi.rs"));
    self::gen_bindings("wrappers/gfx.h", &out_dir.join("sokol_gfx_ffi.rs"));

    self::compile("src/sokol.c", debug);
}

/// Helper for selecting Sokol renderer
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
            // Select default renderer
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

    pub fn set_bindgen_flag(&self, b: bindgen::Builder) -> bindgen::Builder {
        match self {
            Self::D3D11 => b.clang_arg("-DSOKOL_D3D11"),
            Self::Metal => b.clang_arg("-DSOKOL_METAL"),
            Self::GlCore33 => b.clang_arg("-DSOKOL_GLCORE33"),
        }
    }

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

/// Select `.m` file if we're on macOS
fn maybe_select_objective_c(wrapper: &str) -> PathBuf {
    if cfg!(target_os = "macos") {
        PathBuf::from(wrapper).with_extension("m")
    } else {
        PathBuf::from(wrapper)
    }
}

fn gen_bindings(wrapper_str: &str, ffi_output: &Path) {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let sokol_dir = root.join("sokol");

    // Seems like the renderer doesn't change the header declaration though
    let renderer = {
        let build = Build::new();
        let tool = build.try_get_compiler().unwrap();
        let is_msvc = tool.is_like_msvc();
        Renderer::get(is_msvc)
    };

    // ----------------------------------------
    // Generate FFI

    let wrapper = self::maybe_select_objective_c(wrapper_str);

    let bindings = {
        let b = bindgen::builder();
        let b = b.header(format!("{}", wrapper.display()));
        let b = b.clang_arg(format!("-I{}", sokol_dir.display()));
        let b = renderer.set_bindgen_flag(b);
        let b = b.derive_default(true);
        b.generate().unwrap()
    };

    bindings
        .write_to_file(ffi_output)
        .expect("Couldn't write bindings!");
}

/// Compiles the given `wrapper` file and create FFI to it
fn compile(src_path_str: &str, will_set_debug_flags: bool) {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let sokol_dir = root.join("sokol");

    // ----------------------------------------
    // Get metadata

    let mut build = Build::new();
    let tool = build.try_get_compiler().unwrap();
    let is_msvc = tool.is_like_msvc();
    let renderer = Renderer::get(is_msvc);

    // ----------------------------------------
    // Set up compiler flags

    build.include(&sokol_dir);
    let src = self::maybe_select_objective_c(src_path_str);
    build.file(&src);

    renderer.set_cflag(&mut build);
    renderer.link();

    // MacOS: need ARC
    if cfg!(target_os = "macos") {
        build.flag("-fobjc-arc");
    }

    // x86_64-pc-windows-gnu: additional compile/link flags
    if cfg!(target_os = "windows") && !is_msvc {
        build
            .flag("-D_WIN32_WINNT=0x0601")
            .flag_if_supported("-Wno-cast-function-type")
            .flag_if_supported("-Wno-sign-compare")
            .flag_if_supported("-Wno-unknown-pragmas");

        // also link some libraries here..
        println!("cargo:rustc-link-lib=static=gdi32");
        println!("cargo:rustc-link-lib=static=ole32");
    }

    if will_set_debug_flags {
        build.flag("-D_DEBUG").flag("-DSOKOL_DEBUG");
    }

    // ----------------------------------------
    // Compile

    build.flag("-DSOKOL_IMPL");
    build.compile("sokol");

    // ----------------------------------------
    // Link platform-dependent libraries

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
