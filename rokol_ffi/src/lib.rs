//! Rust FFI to Sokol modules

pub mod app {
    //! `sokol_app.h`

    // suppress all warnings
    #![allow(warnings)]

    // Include generated bindings
    include!(concat!(env!("OUT_DIR"), "/sokol_app_ffi.rs"));
}

pub mod gfx {
    //! `sokol_gfx.h`

    // suppress all warnings
    #![allow(warnings)]

    // Include generated bindings
    include!(concat!(env!("OUT_DIR"), "/sokol_gfx_ffi.rs"));
}

pub mod glue {
    //! `sokol_glue.h`

    extern "C" {
        pub fn sapp_sgcontext() -> crate::gfx::sg_context_desc;
    }
}
