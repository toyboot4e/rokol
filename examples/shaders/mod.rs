//! Shader files
//!
//! Conditionally embedded to the source code.
//!
//! D3D11 is not supported (yet).

macro_rules! c_str {
    ($path:expr) => {
        concat!(include_str!($path), "\0");
    };
}

#[cfg(rokol_gfx = "glcore33")]
mod glsl {
    pub static SIMPLE_VS: &str = c_str!("glsl/simple_vs.glsl");
    pub static SIMPLE_FS: &str = c_str!("glsl/simple_fs.glsl");
}

#[cfg(rokol_gfx = "glcore33")]
pub use self::glsl::*;

#[cfg(rokol_gfx = "metal")]
mod metal {
    pub static SIMPLE_VS: &str = c_str!("metal/simple_vs.metal");
    pub static SIMPLE_FS: &str = c_str!("metal/simple_fs.metal");
}

#[cfg(rokol_gfx = "metal")]
pub use self::metal::*;

pub fn make_simple_shader() -> rokol::gfx::Shader {
    unsafe { rokol::gfx::make_shader_static(SIMPLE_VS, SIMPLE_FS) }
}
