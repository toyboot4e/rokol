//! Rust FFI to [Sokol], only for [Rokol]
//!
//! [Sokol]: https://github.com/floooh/sokol
//! [Rokol]: https://github.com/toyboot4e/rokol

pub mod app {
    //! `sokol_app.h`

    // suppress all warnings
    #![allow(warnings)]

    // Include generated bindings
    include!("ffi/sokol_app.rs");
}

pub mod gfx {
    //! `sokol_gfx.h`

    // suppress all warnings
    #![allow(warnings)]

    // Include generated bindings
    include!("ffi/sokol_gfx.rs");
}

pub mod glue {
    //! `sokol_glue.h`

    extern "C" {
        pub fn sapp_sgcontext() -> crate::gfx::sg_context_desc;
    }
}
