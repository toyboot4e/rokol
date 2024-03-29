//! Build script of `rokol-ffi`

// TODO: refactor cfg! with fn
// NOTE: in Crates.io, the file system is read-only and writing to `src/ffi` can fail.

use std::{
    env,
    path::{Path, PathBuf},
};

use cc::Build;

fn main() {
    // update the bindings only when we update `sokol` or `wrappers`
    println!("cargo:rerun-if-changed=sokol");
    println!("cargo:rerun-if-changed=wrappers");

    if !cfg!(feature = "impl-app") && !cfg!(feature = "impl-gfx") {
        panic!("use at least one of `impl-app` or `impl-gfx` feature flag");
    }

    let mut build = Build::new();

    let is_debug = env::var("ROKOL_FORCE_DEBUG").ok().is_some() || env::var("DEBUG").ok().is_some();

    let is_msvc = {
        let tool = build.try_get_compiler().unwrap();
        tool.is_like_msvc()
    };

    let renderer = Renderer::select(is_msvc);
    // emit `DEP_SOKOL_GFX_<Renderer>`
    renderer.emit_cargo_metadata();

    // generate bindings
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let args = &[
        format!("-I{}", root.join("sokol").display()),
        format!("-I{}", root.join("sokol/util").display()),
        format!("-D{}", renderer.sokol_flag_name()),
    ];

    if cfg!(feature = "impl-app") {
        self::gen_bindings(
             root.join("wrappers/rokol_app.h"),
             root.join("src/app.rs"),
             args,
             "//! Rust FFI to [sokol_app.h](https://github.com/floooh/sokol/blob/master/sokol_app.h)",
             |b| b.derive_default(true).derive_partialeq(true).derive_eq(true)
                     .default_enum_style(bindgen::EnumVariation::Rust {
                 non_exhaustive: true,
             })
        );
    }

    if cfg!(feature = "impl-gfx") {
        self::gen_bindings(
             root.join("wrappers/rokol_gfx.h"),
             root.join("src/gfx.rs"),
             args,
             "//! Rust FFI to [sokol_gfx.h](https://github.com/floooh/sokol/blob/master/sokol_gfx.h)",
             |b| b.derive_default(true).derive_partialeq(true).derive_eq(true)
                     .default_enum_style(bindgen::EnumVariation::Rust {
                 non_exhaustive: true,
             })
         );
    }

    // compile and link to them
    self::compile(&mut build, is_msvc, &renderer, is_debug);
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
        // set renderer defined with feature flag
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

/// Generates Rust FFI using a wrapper header file
fn gen_bindings(
    wrapper: impl AsRef<Path>,
    dst: impl AsRef<Path>,
    args: impl IntoIterator<Item = impl AsRef<str>>,
    docstring: &str,
    mut setup_builder: impl FnMut(bindgen::Builder) -> bindgen::Builder,
) {
    let gen = bindgen::Builder::default()
        .header(format!("{}", wrapper.as_ref().display()))
        .clang_args(args)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

    let gen = setup_builder(gen);
    let gen = gen
        .raw_line(docstring)
        .raw_line("")
        .raw_line(r"#![allow(warnings)]");

    let gen = gen.generate().unwrap_or_else(|err| {
        panic!(
            "Unable to generate bindings for `{}`. Original error {:?}",
            dst.as_ref().display(),
            err
        )
    });

    // it's `ok` to fail on crates.io
    gen.write_to_file(dst).ok();
}

fn compile(build: &mut Build, is_msvc: bool, renderer: &Renderer, will_set_debug_flags: bool) {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // ----------------------------------------
    // Set up compiler flags

    // NOTE: Not found on Windows?
    build.flag("-std=c99");

    // -Isokol
    build.include(&root.join("sokol"));
    // -Isokol/util
    build.include(&root.join("sokol/util"));

    // NOTE: On windows, sokol_app.h must be compiled as Objective-C
    if cfg!(feature = "impl-app") && cfg!(feature = "impl-gfx") {
        if cfg!(target_os = "macos") {
            build.file(root.join("wrappers/rokol_glue_impl.m"));
        } else {
            build.file(root.join("wrappers/rokol_glue_impl.c"));
        }
    } else {
        if cfg!(feature = "impl-app") {
            if cfg!(target_os = "macos") {
                build.file(root.join("wrappers/rokol_app_impl.m"));
            } else {
                build.file(root.join("wrappers/rokol_app_impl.c"));
            }
        }
        if cfg!(feature = "impl-gfx") {
            build.file(root.join("wrappers/rokol_gfx_impl.c"));
        }
    }

    // TODO: supply only required flags for requested header

    // #define SOKOL_<RENDERER>
    build.flag(&format!("-D{}", renderer.sokol_flag_name()));

    // Compile as Objective-C
    if cfg!(target_os = "macos") {
        // This time, it's named as `rokol_app_impl.m` and it's already Objective-C.
        // build.flag("-fobjc-arc");
        // build.flag("-ObjC");
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

    // TODO: link properly (for each feature combination)

    if cfg!(feature = "impl-gfx") {
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

    if cfg!(feature = "impl-app") {
        //
    }
}
