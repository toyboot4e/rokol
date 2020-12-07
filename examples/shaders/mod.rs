//! Shaders
//!
//! Shader files are conditionally embedded to the source code.
//!
//! Set `build.rs` for the conditional compiltion information.
//!
//! NOTE: Be sure to set uniform names (or maybe fail).

use rokol::gfx::{self as rg, BakedResource, Shader};

/// Creates a null-terminated string from a file
macro_rules! c_str {
    ($s:expr) => {
        concat!($s, "\0");
    };
}

fn desc(vs_fs: &[&str; 2]) -> rokol::gfx::ShaderDesc {
    unsafe { rokol::gfx::shader_desc(vs_fs[0], vs_fs[1]) }
}

pub fn triangle() -> rokol::gfx::Shader {
    Shader::create(&desc(&TRIANGLE))
}

pub fn quad() -> rokol::gfx::Shader {
    Shader::create(&desc(&QUAD))
}

pub fn texture() -> rokol::gfx::Shader {
    let mut desc = desc(&TEX);

    desc.fs.images[0] = rg::ShaderImageDesc {
        type_: rg::ImageType::Dim2 as u32,
        ..Default::default()
    };

    Shader::create(&desc)
}

pub fn texture_multi() -> rokol::gfx::Shader {
    let mut desc = desc(&TEX_MULTI);

    desc.fs.images[0] = rg::ShaderImageDesc {
        type_: rg::ImageType::Dim2 as u32,
        name: c_str!("tex1").as_ptr() as *const _,
        ..Default::default()
    };
    desc.fs.images[1] = rg::ShaderImageDesc {
        type_: rg::ImageType::Dim2 as u32,
        name: c_str!("tex2").as_ptr() as *const _,
        ..Default::default()
    };

    Shader::create(&desc)
}

pub fn texcube() -> rokol::gfx::Shader {
    let mut desc = desc(&TEX_CUBE);

    desc.vs.uniform_blocks[0] = {
        let mut block = rg::ShaderUniformBlockDesc {
            size: std::mem::size_of::<glam::Mat4>() as i32,
            ..Default::default()
        };
        block.uniforms[0] = rg::ShaderUniformDesc {
            type_: rg::UniformType::Mat4 as u32,
            name: c_str!("mvp").as_ptr() as *const _,
            ..Default::default()
        };
        block
    };

    desc.fs.images[0] = rg::ShaderImageDesc {
        type_: rg::ImageType::Dim2 as u32,
        name: c_str!("tex").as_ptr() as *const _,
        ..Default::default()
    };

    Shader::create(&desc)
}

pub fn texcube_multi() -> rokol::gfx::Shader {
    let mut desc = desc(&TEX_CUBE_MULTI);

    desc.vs.uniform_blocks[0] = {
        let mut block = rg::ShaderUniformBlockDesc {
            size: std::mem::size_of::<glam::Mat4>() as i32,
            ..Default::default()
        };
        block.uniforms[0] = rg::ShaderUniformDesc {
            type_: rg::UniformType::Mat4 as u32,
            name: c_str!("mvp").as_ptr() as *const _,
            ..Default::default()
        };
        block
    };

    desc.fs.images[0] = rg::ShaderImageDesc {
        type_: rg::ImageType::Dim2 as u32,
        name: c_str!("tex1").as_ptr() as *const _,
        ..Default::default()
    };

    desc.fs.images[1] = rg::ShaderImageDesc {
        type_: rg::ImageType::Dim2 as u32,
        name: c_str!("tex2").as_ptr() as *const _,
        ..Default::default()
    };

    Shader::create(&desc)
}

// --------------------------------------------------------------------------------
// Shader files

macro_rules! shd_set {
    ($a:expr, $b:expr) => {
        [
            concat!(include_str!($a), "\0"),
            concat!(include_str!($b), "\0"),
        ]
    };
}

#[cfg(rokol_gfx = "glcore33")]
macro_rules! def_shd {
    ($name:ident, $file:expr) => {
        static $name: [&str; 2] = [
            concat!(include_str!(concat!("glsl/", $file, ".vert")), "\0"),
            concat!(include_str!(concat!("glsl/", $file, ".frag")), "\0"),
        ];
    };
}

#[cfg(rokol_gfx = "metal")]
macro_rules! def_shd {
    ($name:ident, $file:expr) => {
        static $name: [&str; 2] = [
            concat!(include_str!(concat!("metal/", $file, "_vs.metal")), "\0"),
            concat!(include_str!(concat!("metal/", $file, "_fs.metal")), "\0"),
        ];
    };
}

#[cfg(rokol_gfx = "d3d11")]
macro_rules! def_shd {
    ($name:ident, $file:expr) => {
        static $name: [&str; 2] = [
            concat!(include_str!(concat!("d3d11/", $file, "_vs.hlsl")), "\0"),
            concat!(include_str!(concat!("d3d11/", $file, "_fs.hlsl")), "\0"),
        ]
    };
}

def_shd!(TRIANGLE, "triangle");
def_shd!(QUAD, "quad");
def_shd!(TEX, "texture");
def_shd!(TEX_MULTI, "texture_multi");
def_shd!(TEX_CUBE, "texcube");
def_shd!(TEX_CUBE_MULTI, "texcube_multi");

// #[cfg(rokol_gfx = "glcore33")]
// mod files {
//     pub static TRIANGLE: [&str; 2] = shd_set!("glsl/triangle.vert", "glsl/triangle.frag");
//     pub static QUAD: [&str; 2] = shd_set!("glsl/quad.vert", "glsl/quad.frag");
//     pub static TEXTURE: [&str; 2] = shd_set!("glsl/texture.vert", "glsl/texture.frag");
//     pub static TEX_MULTI: [&str; 2] =
//         shd_set!("glsl/texture_multi.vert", "glsl/texture_multi.frag");
//     pub static TEX_CUBE: [&str; 2] = shd_set!("glsl/texcube.vert", "glsl/texcube.frag");
//     pub static TEX_CUBE_MULTI: [&str; 2] =
//         shd_set!("glsl/texcube_multi.vert", "glsl/texcube_multi.frag");
// }

// #[cfg(rokol_gfx = "metal")]
// mod files {
//     pub static TRIANGLE: [&str; 2] = shd_set!("metal/triangle_vs.metal", "metal/triangle_fs.metal");
//     pub static QUAD: [&str; 2] = shd_set!("metal/quad_vs.metal", "metal/quad_fs.metal");
// }

// #[cfg(rokol_gfx = "d3d11")]
// mod files {}
