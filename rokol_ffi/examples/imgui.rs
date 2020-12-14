// //! Just for testing link to `sokol_gfx.h` and `cimgui`

use rokol_ffi::imgui as ri;

fn main() {
    // TODO: fix the link error
    unsafe {
        // ri::simgui_render();
        ri::simgui_setup(&ri::simgui_desc_t::default());
    }
}
