//! Shader files
//!
//! Conditionally embedded to the source code.
//!
//! D3D11 is not supported (yet).

#[cfg(rokol_gfx = "glcore33")]
mod glsl {
    pub static SIMPLE_VS: &str = include_str!("glsl/simple_vs.glsl");
    pub static SIMPLE_FS: &str = include_str!("glsl/simple_fs.glsl");
}
#[cfg(rokol_gfx = "glcore33")]
pub use self::glsl::*;

#[cfg(rokol_gfx = "metal")]
mod metal {
    pub static SIMPLE_VS: &str = include_str!("metal/simple_vs.metal");
    pub static SIMPLE_FS: &str = include_str!("metal/simple_fs.metal");
}
#[cfg(rokol_gfx = "metal")]
pub use self::metal::*;
