//! ImGUI setup for `sokol_app` and `sokol_gfx`
//!
//! NOTE: `rokol_ffi::imgui::simgui_shutdown` is not included.

use rokol_ffi::imgui as ffi;

pub type ImguiDesc = ffi::simgui_desc_t;

// --------------------------------------------------------------------------------
// `sokol_imgui.h`

pub fn setup(desc: &ImguiDesc) {
    unsafe {
        ffi::simgui_setup(desc as *const _);
    }
}

pub fn handle_event(ev: &crate::app::RAppEvent) -> bool {
    unsafe { ffi::simgui_handle_event(ev as *const _) }
}

pub fn new_frame(w: u32, h: u32, dt: f64) {
    unsafe {
        ffi::simgui_new_frame(w as i32, h as i32, dt);
    }
}

pub fn render() {
    unsafe {
        ffi::simgui_render();
    }
}
