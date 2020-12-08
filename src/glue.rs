//! Glue code ([`FFI`])
//!
//! [`FFI`]: rokol_ffi::glue

/// For [`crate::gfx::setup`]
///
/// Creates [`crate::gfx::SetupDesc`] considering [`crate::app`].
pub fn app_desc() -> rokol_ffi::gfx::sg_desc {
    let mut desc: rokol_ffi::gfx::sg_desc = Default::default();
    desc.context = unsafe { rokol_ffi::glue::sapp_sgcontext() };
    desc
}
