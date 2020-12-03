//! Build script of `rokol-ffi`
//!
//! See `README.md` for more information.

use std::{
    env,
    path::{Path, PathBuf},
};

use cc::Build;

fn main() {
    // Select one of D3D11, Metal or GlCore33
    println!("cargo:rerun-if-env-changed=ROKOL_RENDERER");

    // generate bindings to `src/ffi`
    let out_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("src/ffi");
    let mut build = Build::new();

    let debug = env::var("ROKOL_FORCE_DEBUG").ok().is_some() || env::var("DEBUG").ok().is_some();
    let is_msvc = {
        let tool = build.try_get_compiler().unwrap();
        tool.is_like_msvc()
    };

    let renderer = Renderer::select(is_msvc);

    renderer.emit_cargo_metadata();

    self::gen_bindings("wrappers/app.h", &out_dir.join("sokol_app.rs"), &renderer);
    self::gen_bindings("wrappers/gfx.h", &out_dir.join("sokol_gfx.rs"), &renderer);

    self::compile(&mut build, is_msvc, &renderer, "wrappers/sokol.c", debug);
}

/// Helper for selecting Sokol renderer
enum Renderer {
    D3D11,
    Metal,
    GlCore33,
}

impl Renderer {
    pub fn select(is_msvc: bool) -> Self {
        // set renderer with environmental variable
        if let Ok(rdr) = env::var("ROKOL_RENDERER") {
            return match rdr.as_str() {
                "D3D11" => Self::D3D11,
                "Metal" => Self::Metal,
                "GlCore33" => Self::GlCore33,
                _ => panic!("ROKOL_RENDERER is invalid: {}", rdr),
            };
        }

        // set renderer via feature
        #[cfg(feature = "force-glcore33")]
        return Self::GlCore33;

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

    /// `-D` flag name
    pub fn sokol_flag_name(&self) -> &str {
        match self {
            Self::D3D11 => "SOKOL_D3D11",
            Self::Metal => "SOKOL_METAL",
            Self::GlCore33 => "SOKOL_GLCORE33",
        }
    }

    /// Provides with an environmental variable `DEP_SOKOL_GFX` to `build.rs` files in crates that
    /// are dependent on `rokol_ffi`
    pub fn emit_cargo_metadata(&self) {
        match self {
            Self::D3D11 => println!("cargo:gfx=\"d3d11\""),
            Self::Metal => println!("cargo:gfx=\"metal\""),
            Self::GlCore33 => println!("cargo:gfx=\"glcore33\""),
        }
    }
}

/// Change extension to `.` on macOS
fn maybe_select_objective_c(wrapper: &str) -> PathBuf {
    if cfg!(target_os = "macos") {
        PathBuf::from(wrapper).with_extension("m")
    } else {
        PathBuf::from(wrapper)
    }
}

fn gen_bindings(wrapper_str: &str, ffi_output: &Path, renderer: &Renderer) {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let sokol_dir = root.join("sokol");

    // ----------------------------------------
    // Generate FFI

    let wrapper = self::maybe_select_objective_c(wrapper_str);

    let bindings = {
        let b = bindgen::builder();
        let b = b.header(format!("{}", wrapper.display()));
        let b = b.clang_arg(format!("-I{}", sokol_dir.display()));
        let b = b.clang_arg(format!("-D{}", renderer.sokol_flag_name()));
        let b = b.derive_default(true);
        b.generate().unwrap()
    };

    bindings
        .write_to_file(ffi_output)
        .expect("Couldn't write bindings!");
}

/// Compiles the given `wrapper` file and create FFI to it
fn compile(
    build: &mut Build,
    is_msvc: bool,
    renderer: &Renderer,
    src_path_str: &str,
    will_set_debug_flags: bool,
) {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let sokol_dir = root.join("sokol");

    // ----------------------------------------
    // Set up compiler flags

    build.include(&sokol_dir);
    let src = self::maybe_select_objective_c(src_path_str);
    build.file(&src);

    build.flag(&format!("-D{}", renderer.sokol_flag_name()));

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
                println!("cargo:rustc-link-lib=framework=OpenGL");
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
