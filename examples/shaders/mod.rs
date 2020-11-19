//! Shaders
//!
//! Shader files are conditionally embedded to the source code.
//!
//! ```sh no_run
//! env ROKOL_RENDERER=GlCore33 cargo run --example quad
//! ```

use {rokol::gfx as rg, std::mem::size_of};

macro_rules! c_str {
    ($path:expr) => {
        concat!(include_str!($path), "\0");
    };
}

fn make(vs: &str, fs: &str) -> rokol::gfx::Shader {
    let desc = unsafe { rokol::gfx::shader_desc(vs, fs) };
    rg::make_shader(&desc)
}

pub fn make_triangle_shader() -> rokol::gfx::Shader {
    make(files::TRIANGLE_VS, files::TRIANGLE_FS)
}

pub fn make_quad_shader() -> rokol::gfx::Shader {
    make(files::QUAD_VS, files::QUAD_FS)
}

pub fn make_texture_shader() -> rokol::gfx::Shader {
    let mut desc = unsafe { rokol::gfx::shader_desc(files::TEXTURE_VS, files::TEXTURE_FS) };

    desc.vs.uniform_blocks[0] = {
        let mut block = rg::ShaderUniformBlockDesc {
            ..Default::default()
        };
        block.uniforms[0] = rg::ShaderUniformDesc {
            type_: rg::UniformType::Mat4 as u32,
            ..Default::default()
        };
        block.size += 16 * size_of::<f32>() as i32;
        block
    };

    desc.fs.images[0] = rg::ShaderImageDesc {
        type_: rg::ImageType::Dim2 as u32,
        ..Default::default()
    };

    rg::make_shader(&desc)
}

// --------------------------------------------------------------------------------
// Shader files

#[cfg(rokol_gfx = "glcore33")]
mod files {
    pub static TRIANGLE_VS: &str = c_str!("glsl/triangle_vs.glsl");
    pub static TRIANGLE_FS: &str = c_str!("glsl/triangle_fs.glsl");

    pub static QUAD_VS: &str = c_str!("glsl/quad_vs.glsl");
    pub static QUAD_FS: &str = c_str!("glsl/quad_fs.glsl");

    pub static TEXTURE_VS: &str = c_str!("glsl/texture_vs.glsl");
    pub static TEXTURE_FS: &str = c_str!("glsl/texture_fs.glsl");
}

#[cfg(rokol_gfx = "metal")]
mod files {
    pub static TRIANGLE_VS: &str = c_str!("metal/triangle_vs.metal");
    pub static TRIANGLE_FS: &str = c_str!("metal/triangle_fs.metal");

    pub static QUAD_VS: &str = c_str!("metal/quad_vs.metal");
    pub static QUAD_FS: &str = c_str!("metal/quad_fs.metal");

    pub static TEXTURE_VS: &str = "<unimplemented shader>";
    pub static TEXTURE_FS: &str = "<unimplemented shader>";
}

#[cfg(rokol_gfx = "d3d11")]
mod files {
    pub static triangle_VS: &str = "<unimplemented shader>";
    pub static triangle_FS: &str = "<unimplemented shader>";

    pub static QUAD_VS: &str = "<unimplemented shader>";
    pub static QUAD_FS: &str = "<unimplemented shader>";

    pub static TEXTURE_VS: &str = "<unimplemented shader>";
    pub static TEXTURE_FS: &str = "<unimplemented shader>";
}
