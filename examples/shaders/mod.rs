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

pub fn make_triangle_shader() -> rokol::gfx::Shader {
    make(files::TRIANGLE_VS, files::TRIANGLE_FS)
}

pub fn make_quad_shader() -> rokol::gfx::Shader {
    make(files::QUAD_VS, files::QUAD_FS)
}

// --------------------------------------------------------------------------------
// Shader files

#[cfg(rokol_gfx = "glcore33")]
mod files {
    pub static TRIANGLE_VS: &str = c_str!("glsl/triangle_vs.glsl");
    pub static TRIANGLE_FS: &str = c_str!("glsl/triangle_fs.glsl");

    pub static QUAD_VS: &str = c_str!("glsl/quad_vs.glsl");
    pub static QUAD_FS: &str = c_str!("glsl/quad_fs.glsl");
}

#[cfg(rokol_gfx = "metal")]
mod files {
    pub static TRIANGLE_VS: &str = c_str!("metal/triangle_vs.metal");
    pub static TRIANGLE_FS: &str = c_str!("metal/triangle_fs.metal");

    pub static QUAD_VS: &str = c_str!("metal/quad_vs.metal");
    pub static QUAD_FS: &str = c_str!("metal/quad_fs.metal");
}

#[cfg(rokol_gfx = "d3d11")]
mod files {
    pub static triangle_VS: &str = "<unimplemented shader>";
    pub static triangle_FS: &str = "<unimplemented shader>";

    pub static QUAD_VS: &str = "<unimplemented shader>";
    pub static QUAD_FS: &str = "<unimplemented shader>";
}
