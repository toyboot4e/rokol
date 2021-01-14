//! Build script of `rokol-ffi`

// NOTE: in Crates.io, the file system is read-only and writing to `src/ffi` can fail.

use std::{
    env,
    path::{Path, PathBuf},
};

use cc::Build;

fn main() {
    // update the bindings when we update `sokol` or `wrappers`
    println!("cargo:rerun-if-changed=sokol");
    println!("cargo:rerun-if-changed=wrappers");

    let mut build = Build::new();

    let debug = env::var("ROKOL_FORCE_DEBUG").ok().is_some() || env::var("DEBUG").ok().is_some();
    let is_msvc = {
        let tool = build.try_get_compiler().unwrap();
        tool.is_like_msvc()
    };

    let renderer = Renderer::select(is_msvc);
    // emit `DEP_SOKOL_GFX_<Renderer>`
    renderer.emit_cargo_metadata();

    // generate bindings
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    self::gen_bindings(
        "wrappers/rokol_app.h",
        &root.join("src/app.rs"),
        &renderer,
        "[sokol_app.h](https://github.com/floooh/sokol/blob/master/sokol_app.h)",
    );

    self::gen_bindings(
        "wrappers/rokol_gfx.h",
        &root.join("src/gfx.rs"),
        &renderer,
        "[sokol_gfx.h](https://github.com/floooh/sokol/blob/master/sokol_gfx.h)",
    );

    // compile and link to them
    self::compile(
        &mut build,
        is_msvc,
        &renderer,
        "wrappers/rokol_impl.c",
        debug,
    );
}

/// Helper for selecting Sokol renderer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Renderer {
    D3D11,
    Metal,
    GlCore33,
}

impl Renderer {
    pub fn select(is_msvc: bool) -> Self {
        // set renderer defined by feature
        if cfg!(feature = "glcore33") {
            Self::GlCore33
        } else if cfg!(feature = "metal") {
            Self::Metal
        } else if cfg!(feature = "d3d11") {
            Self::D3D11
        } else {
            // select default renderer
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

    /// `-D` flag name
    pub fn sokol_flag_name(&self) -> &str {
        match self {
            Self::D3D11 => "SOKOL_D3D11",
            Self::Metal => "SOKOL_METAL",
            Self::GlCore33 => "SOKOL_GLCORE33",
        }
    }

    /// Provides with an environmental variable `DEP_SOKOL_GFX` to `build.rs` files in crates that
    /// are directly dependent on `rokol_ffi`
    pub fn emit_cargo_metadata(&self) {
        match self {
            Self::D3D11 => println!("cargo:gfx=\"d3d11\""),
            Self::Metal => println!("cargo:gfx=\"metal\""),
            Self::GlCore33 => println!("cargo:gfx=\"glcore33\""),
        }
    }
}

fn new_bindgen(wrapper_str: &str, renderer: &Renderer, ffi_name: &str) -> bindgen::Builder {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let b = bindgen::builder();

    let b = b.clang_arg(format!("-I{}", root.join("sokol").display()));
    let b = b.clang_arg(format!("-I{}", root.join("sokol/util").display()));

    let b = b.header(format!("{}", wrapper_str));
    let b = b.clang_arg(format!("-D{}", renderer.sokol_flag_name()));

    let b = b.derive_default(true);

    // let b = b.disable_header_comment();
    let b = b.raw_line(format!("//! Rust FFI to {}", ffi_name));
    let b = b.raw_line("");
    let b = b.raw_line("#![allow(warnings)]");

    b
}

fn gen_bindings(wrapper_str: &str, ffi_output: &Path, renderer: &Renderer, ffi_name: &str) {
    let gen = new_bindgen(wrapper_str, renderer, ffi_name);
    gen.generate().unwrap().write_to_file(ffi_output).ok();
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

    // ----------------------------------------
    // Set up compiler flags

    build.flag("-std=c99");

    // -Isokol
    build.include(&root.join("sokol"));
    // -Isokol/util
    build.include(&root.join("sokol/util"));

    build.file(PathBuf::from(src_path_str));

    // #define SOKOL_<RENDERER>
    build.flag(&format!("-D{}", renderer.sokol_flag_name()));

    if cfg!(target_os = "macos") {
        // compile as Objective-C
        // build.flag("-fobjc-arc");
        build.flag("-ObjC");
    }

    if cfg!(target_os = "linux") {
        build.flag("-pthread"); // ?
    }

    // if cfg!(target_os = "windows") && !is_msvc {
    //     build
    //         .flag("-D_WIN32_WINNT=0x0601")
    //         .flag_if_supported("-Wno-cast-function-type")
    //         .flag_if_supported("-Wno-sign-compare")
    //         .flag_if_supported("-Wno-unknown-pragmas");
    // }

    if will_set_debug_flags {
        build.flag("-D_DEBUG").flag("-DSOKOL_DEBUG");
    }

    // ----------------------------------------
    // Compile

    // libsokol.a
    build.compile("sokol");

    // ----------------------------------------
    // Link platform-dependent libraries

    if cfg!(target_os = "windows") && !is_msvc {
        // println!("cargo:rustc-link-lib=static=gdi32");
        // println!("cargo:rustc-link-lib=static=ole32");
    }

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=Cocoa");
        println!("cargo:rustc-link-lib=framework=QuartzCore");
        // println!("cargo:rustc-link-lib=framework=Quartz");
        // println!("cargo:rustc-link-lib=framework=Foundation");

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
    }

    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=dylib=GL");
        println!("cargo:rustc-link-lib=dylib=X11");
        println!("cargo:rustc-link-lib=dylib=Xi");
        println!("cargo:rustc-link-lib=dylib=Xcursor");
        println!("cargo:rustc-link-lib=dylib=dl");
        println!("cargo:rustc-link-lib=dylib=pthread");
        // println!("cargo:rustc-link-lib=dylib=m");
    }
}
