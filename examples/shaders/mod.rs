/*!

Shaders

Shader files are conditionally embedded to the source code.

See `rokol/build.rs` for the conditional compiltion information.
*/

#![allow(unused)]

// NOTE: Be sure to set uniform names (or maybe fail).
// TODO: compile while allowing non-existing shader file

use rokol::gfx::{self as rg, BakedResource, Shader};

/// Creates a null-terminated string from static string
macro_rules! c_str {
    ($s:expr) => {
        concat!($s, "\0");
    };
}

fn gen(vs_fs: &[impl AsRef<str>; 2], f: impl Fn(&mut rg::ShaderDesc)) -> rg::Shader {
    let mut desc = unsafe { rokol::gfx::shader_desc(vs_fs[0].as_ref(), vs_fs[1].as_ref()) };
    f(&mut desc);
    Shader::create(&desc)
}

macro_rules! embed_shd {
    ($vs:expr, $fs:expr,) => {
        if cfg!(debug_assertions) {
            // debug: dynamically load the shader files
            let dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
                .join("examples/shaders");
            let mut v = std::fs::read_to_string(dir.join($vs)).unwrap();
            v.push('\0');
            let mut f = std::fs::read_to_string(dir.join($fs)).unwrap();
            f.push('\0');
            [v, f]
        } else {
            // release: statically load the shader files
            [
                concat!(include_str!($vs), "\0").to_string(),
                concat!(include_str!($fs), "\0").to_string(),
            ]
        }
    };
}

#[cfg(rokol_gfx = "glcore33")]
macro_rules! def_shd {
    ($file:expr) => {
        embed_shd!(
            concat!("glsl/", $file, ".vs"),
            concat!("glsl/", $file, ".fs"),
        )
    };
}

#[cfg(rokol_gfx = "metal")]
macro_rules! def_shd {
    ($file:expr) => {
        embed_shd!(
            concat!("metal/", $file, "_vs.metal"),
            concat!("metal/", $file, "_fs.metal"),
        )
    };
}

#[cfg(rokol_gfx = "d3d11")]
macro_rules! def_shd {
    ($file:expr) => {
        embed_shd!(
            concat!("d3d11/", $file, "_vs.hlsl"),
            concat!("d3d11/", $file, "_fs.hlsl"),
        )
    };
}

macro_rules! img_type {
    ($name:expr,$ty:expr) => {
        rg::ShaderImageDesc {
            name: c_str!($name).as_ptr() as *const _,
            image_type: $ty as u32,
            ..Default::default()
        }
    };
}

/// Single-value uniform block
macro_rules! ub {
    ($name:expr, $uniform_ty:expr, $size_ty:ty) => {{
        let mut block = rg::ShaderUniformBlockDesc::default();

        block.uniforms[0] = rg::ShaderUniformDesc {
            name: concat!($name, "\0").as_ptr() as *const _,
            type_: $uniform_ty as u32,
            ..Default::default()
        };
        block.size += std::mem::size_of::<$size_ty>() as u64;

        block
    }};
}

pub fn triangle() -> rokol::gfx::Shader {
    gen(&def_shd!("triangle"), |_desc| {})
}

pub fn quad() -> rokol::gfx::Shader {
    gen(&def_shd!("quad"), |_desc| {})
}

pub fn texture() -> rokol::gfx::Shader {
    gen(&def_shd!("texture"), |desc| {
        desc.fs.images[0] = img_type!("tex", rg::ImageType::Dim2);
    })
}

pub fn texture_multi() -> rokol::gfx::Shader {
    gen(&def_shd!("texture_multi"), |desc| {
        desc.fs.images[0] = img_type!("tex1", rg::ImageType::Dim2);
        desc.fs.images[1] = img_type!("tex2", rg::ImageType::Dim2);
    })
}

pub fn cube() -> rokol::gfx::Shader {
    gen(&def_shd!("cube"), |desc| {
        desc.fs.images[0] = img_type!("tex", rg::ImageType::Dim2);
        desc.vs.uniform_blocks[0] = ub!("mvp", rg::UniformType::Mat4, glam::Mat4);
    })
}

pub fn cube_multi() -> rokol::gfx::Shader {
    gen(&def_shd!("cube_multi"), |desc| {
        desc.fs.images[0] = img_type!("tex1", rg::ImageType::Dim2);
        desc.fs.images[1] = img_type!("tex2", rg::ImageType::Dim2);
        desc.vs.uniform_blocks[0] = ub!("mvp", rg::UniformType::Mat4, glam::Mat4);
    })
}

pub fn more_cubes() -> rokol::gfx::Shader {
    gen(&def_shd!("more_cubes"), |desc| {
        desc.fs.images[0] = img_type!("tex1", rg::ImageType::Dim2);
        desc.fs.images[1] = img_type!("tex2", rg::ImageType::Dim2);
        desc.vs.uniform_blocks[0] = ub!("mvp", rg::UniformType::Mat4, glam::Mat4);
    })
}
