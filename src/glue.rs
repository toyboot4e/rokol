//! `sokol_glue.h`

/// Glue code for creating application considering `sokol_gfx.h`.
///
/// Used in [`crate::app::RApp::init`] to call `crate::gfx::setup`.
pub fn app_desc() -> rokol_ffi::gfx::sg_desc {
    let mut desc: rokol_ffi::gfx::sg_desc = Default::default();
    desc.context = unsafe { rokol_ffi::glue::sapp_sgcontext() };
    desc
}
