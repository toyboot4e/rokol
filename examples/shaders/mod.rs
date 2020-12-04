//! Shaders
//!
//! Shader files are conditionally embedded to the source code.
//!
//! Set `buidlrs` for the conditional compiltion information.

use {
    rokol::gfx::{self as rg, BakedResource, Shader},
    std::mem::size_of,
};

macro_rules! c_str {
    ($path:expr) => {
        concat!(include_str!($path), "\0");
    };
}

fn desc(vs: &str, fs: &str) -> rokol::gfx::ShaderDesc {
    unsafe { rokol::gfx::shader_desc(vs, fs) }
}

pub fn triangle_shader() -> rokol::gfx::Shader {
    Shader::create(&desc(files::TRIANGLE_VS, files::TRIANGLE_FS))
}

pub fn quad_shader() -> rokol::gfx::Shader {
    Shader::create(&desc(files::QUAD_VS, files::QUAD_FS))
}

pub fn texture_shader() -> rokol::gfx::Shader {
    let mut desc = desc(files::TEXTURE_VS, files::TEXTURE_FS);

    desc.fs.images[0] = rg::ShaderImageDesc {
        type_: rg::ImageType::Dim2 as u32,
        ..Default::default()
    };

    Shader::create(&desc)
}

pub fn texcube_shader() -> rokol::gfx::Shader {
    let mut desc = desc(files::TEXCUBE_VS, files::TEXCUBE_FS);

    desc.vs.uniform_blocks[0] = {
        let mut block = rg::ShaderUniformBlockDesc {
            size: 16 * size_of::<f32>() as i32,
            ..Default::default()
        };
        block.uniforms[0] = rg::ShaderUniformDesc {
            type_: rg::UniformType::Mat4 as u32,
            ..Default::default()
        };
        block
    };

    desc.fs.images[0] = rg::ShaderImageDesc {
        type_: rg::ImageType::Dim2 as u32,
        ..Default::default()
    };

    Shader::create(&desc)
}

// --------------------------------------------------------------------------------
// Shader files

#[cfg(rokol_gfx = "glcore33")]
mod files {
    pub static TRIANGLE_VS: &str = c_str!("glsl/triangle.vert");
    pub static TRIANGLE_FS: &str = c_str!("glsl/triangle.frag");

    pub static QUAD_VS: &str = c_str!("glsl/quad.vert");
    pub static QUAD_FS: &str = c_str!("glsl/quad.frag");

    pub static TEXTURE_VS: &str = c_str!("glsl/texture.vert");
    pub static TEXTURE_FS: &str = c_str!("glsl/texture.frag");

    pub static TEXCUBE_VS: &str = c_str!("glsl/texcube.vert");
    pub static TEXCUBE_FS: &str = c_str!("glsl/texcube.frag");
}

#[cfg(rokol_gfx = "metal")]
mod files {
    pub static TRIANGLE_VS: &str = c_str!("metal/triangle_vs.metal");
    pub static TRIANGLE_FS: &str = c_str!("metal/triangle_fs.metal");

    pub static QUAD_VS: &str = c_str!("metal/quad_vs.metal");
    pub static QUAD_FS: &str = c_str!("metal/quad_fs.metal");

    pub static TEXTURE_VS: &str = "<unimplemented shader>";
    pub static TEXTURE_FS: &str = "<unimplemented shader>";

    pub static TEXCUBE_VS: &str = "<unimplemented shader>";
    pub static TEXCUBE_FS: &str = "<unimplemented shader>";
}

#[cfg(rokol_gfx = "d3d11")]
mod files {
    pub static triangle_VS: &str = "<unimplemented shader>";
    pub static triangle_FS: &str = "<unimplemented shader>";

    pub static QUAD_VS: &str = "<unimplemented shader>";
    pub static QUAD_FS: &str = "<unimplemented shader>";

    pub static TEXTURE_VS: &str = "<unimplemented shader>";
    pub static TEXTURE_FS: &str = "<unimplemented shader>";

    pub static TEXCUBE_VS: &str = "<unimplemented shader>";
    pub static TEXCUBE_FS: &str = "<unimplemented shader>";
}
