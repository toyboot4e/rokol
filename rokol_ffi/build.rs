//! Build script of `rokol-ffi`

// TODO: consider read-only file system (crates.io)

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

    // generate bindings to `src/ffi`
    let gen_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("src/ffi");
    self::gen_bindings(
        "wrappers/rokol_app.h",
        &gen_dir.join("sokol_app.rs"),
        &renderer,
    );

    self::gen_bindings(
        "wrappers/rokol_gfx.h",
        &gen_dir.join("sokol_gfx.rs"),
        &renderer,
    );

    {
        let gen = self::new_bindgen("wrappers/rokol_imgui.h", &renderer);
        // Do not whitelist dependent items of whitelisted items
        let gen = gen.whitelist_recursively(false);
        // Only generate bindings to `sokol_imgui` and `sokol_gfx_imgui` items
        let gen = gen.whitelist_type("simgui.*");
        let gen = gen.whitelist_function("simgui.*");
        let gen = gen.whitelist_type("sg_imgui.*");
        let gen = gen.whitelist_function("sg_imgui.*");
        // NOTE: Now, `sokol_imgui.rs` does not compile. Instead, we'll import `sokol_gfx`
        //       items in `lib.rs`.)
        gen.generate()
            .expect("failed to generate FFI to Sokol ImGUI")
            .write_to_file(&gen_dir.join("sokol_imgui.rs"))
            .unwrap();
    }

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
        #[cfg(feature = "glcore33")]
        return Self::GlCore33;

        #[cfg(feature = "metal")]
        return Self::Metal;

        #[cfg(feature = "d3d11")]
        return Self::D3d11;

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

fn new_bindgen(wrapper_str: &str, renderer: &Renderer) -> bindgen::Builder {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let b = bindgen::builder();

    let b = b.clang_arg(format!("-I{}", root.join("sokol").display()));
    let b = b.clang_arg(format!("-I{}", root.join("sokol/util").display()));
    // `imgui-sys` contains `cimgui`, which is exported with their `build.rs`
    let cimgui = PathBuf::from(env::var("DEP_IMGUI_THIRD_PARTY").unwrap());
    let b = b.clang_arg(format!("-I{}", cimgui.display()));

    let b = b.header(format!("{}", wrapper_str));
    let b = b.clang_arg(format!("-D{}", renderer.sokol_flag_name()));

    let b = b.derive_default(true);
    b
}

fn gen_bindings(wrapper_str: &str, ffi_output: &Path, renderer: &Renderer) {
    let gen = new_bindgen(wrapper_str, renderer);
    gen.generate()
        .unwrap()
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

    // ----------------------------------------
    // Set up compiler flags

    // -Isokol
    build.include(&root.join("sokol"));
    // -Isokol/util
    build.include(&root.join("sokol/util"));

    // #include "cimugi.h"
    {
        // https://github.com/imgui-rs/imgui-rs/blob/master/imgui-sys/build.rs#L30
        // https://doc.rust-lang.org/cargo/reference/build-scripts.html#-sys-packages
        let cimgui = PathBuf::from(env::var("DEP_IMGUI_THIRD_PARTY").unwrap());
        build.include(cimgui);
        let imgui = PathBuf::from(env::var("DEP_IMGUI_THIRD_PARTY").unwrap()).join("imgui");
        build.include(imgui);
    }

    build.file(PathBuf::from(src_path_str));

    // #define SOKOL_<RENDERER>
    build.flag(&format!("-D{}", renderer.sokol_flag_name()));

    // MacOS: need ARC
    if cfg!(target_os = "macos") {
        build.flag("-fobjc-arc");
        build.flag("-std=c99");
        build.flag("-ObjC");
    }

    // x86_64-pc-windows-gnu: additional compile flags
    if cfg!(target_os = "windows") && !is_msvc {
        build
            .flag("-D_WIN32_WINNT=0x0601")
            .flag_if_supported("-Wno-cast-function-type")
            .flag_if_supported("-Wno-sign-compare")
            .flag_if_supported("-Wno-unknown-pragmas");
    }

    if will_set_debug_flags {
        build.flag("-D_DEBUG").flag("-DSOKOL_DEBUG");
    }

    // ----------------------------------------
    // Compile

    // TODO: link to ImGUI
    build.compile("sokol");

    // ----------------------------------------
    // Link platform-dependent libraries

    // x86_64-pc-windows-gnu
    if cfg!(target_os = "windows") && !is_msvc {
        println!("cargo:rustc-link-lib=static=gdi32");
        println!("cargo:rustc-link-lib=static=ole32");
    }

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
