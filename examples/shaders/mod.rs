//! Shaders
//!
//! Shader files are conditionally embedded to the source code.
//!
//! Set `build.rs` for the conditional compiltion information.

// NOTE: Be sure to set uniform names (or maybe fail).
// TODO: compile while allowing non-existing shader file

use rokol::gfx::{self as rg, BakedResource, Shader};

/// Creates a null-terminated string from a file
macro_rules! c_str {
    ($s:expr) => {
        concat!($s, "\0");
    };
}

fn gen(vs_fs: &[&str; 2], f: impl Fn(&mut rg::ShaderDesc)) -> rg::Shader {
    let mut desc = unsafe { rokol::gfx::shader_desc(vs_fs[0], vs_fs[1]) };
    f(&mut desc);
    Shader::create(&desc)
}

pub fn triangle() -> rokol::gfx::Shader {
    gen(&TRIANGLE, |_desc| {})
}

pub fn quad() -> rokol::gfx::Shader {
    gen(&QUAD, |_desc| {})
}

pub fn texture() -> rokol::gfx::Shader {
    gen(&QUAD, |desc| {
        desc.fs.images[0] = rg::ShaderImageDesc {
            type_: rg::ImageType::Dim2 as u32,
            ..Default::default()
        };
    })
}

pub fn texture_multi() -> rokol::gfx::Shader {
    gen(&TEX_MULTI, |desc| {
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
    })
}

pub fn texcube() -> rokol::gfx::Shader {
    gen(&TEX_CUBE, |desc| {
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
    })
}

pub fn texcube_multi() -> rokol::gfx::Shader {
    gen(&TEX_CUBE_MULTI, |desc| {
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
    })
}

// --------------------------------------------------------------------------------
// Shader files

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
