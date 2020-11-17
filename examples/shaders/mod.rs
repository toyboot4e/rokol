//! Shaders
//!
//! Shader files are conditionally embedded to the source code.
//!
//! ```sh no_run
//! env ROKOL_RENDERER=GlCore33 cargo run --example quad
//! ```

macro_rules! c_str {
    ($path:expr) => {
        concat!(include_str!($path), "\0");
    };
}

fn make(fs: &str, vs: &str) -> rokol::gfx::Shader {
    unsafe { rokol::gfx::make_shader_static(fs, vs) }
}

pub fn make_simple_shader() -> rokol::gfx::Shader {
    make(files::SIMPLE_VS, files::SIMPLE_FS)
}

pub fn make_quad_shader() -> rokol::gfx::Shader {
    make(files::QUAD_VS, files::QUAD_FS)
}

// --------------------------------------------------------------------------------
// Shader files

#[cfg(rokol_gfx = "glcore33")]
mod files {
    pub static SIMPLE_VS: &str = c_str!("glsl/simple_vs.glsl");
    pub static SIMPLE_FS: &str = c_str!("glsl/simple_fs.glsl");

    pub static QUAD_VS: &str = c_str!("glsl/quad_vs.glsl");
    pub static QUAD_FS: &str = c_str!("glsl/quad_fs.glsl");
}

#[cfg(rokol_gfx = "metal")]
mod files {
    pub static SIMPLE_VS: &str = c_str!("metal/simple_vs.metal");
    pub static SIMPLE_FS: &str = c_str!("metal/simple_fs.metal");

    pub static QUAD_VS: &str = "<unimplemented shader>";
    pub static QUAD_FS: &str = "<unimplemented shader>";
}

#[cfg(rokol_gfx = "d3d11")]
mod files {
    pub static SIMPLE_VS: &str = "<unimplemented shader>";
    pub static SIMPLE_FS: &str = "<unimplemented shader>";

    pub static QUAD_VS: &str = "<unimplemented shader>";
    pub static QUAD_FS: &str = "<unimplemented shader>";
}
