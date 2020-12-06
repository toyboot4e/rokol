//! Rust FFI to [Sokol], only for [Rokol]
//!
//! Last update: Dec 3, 2020 ([commit]).
//!
//! [Sokol]: https://github.com/floooh/sokol
//! [Rokol]: https://github.com/toyboot4e/rokol
//!
//! [commit]: https://github.com/floooh/sokol/commit/64a6f2e2fac607ddcd4ccc5bd8bcd25946293550

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

    include!("ffi/sokol_gfx.rs");
}

pub mod glue {
    //! `sokol_glue.h`

    extern "C" {
        pub fn sapp_sgcontext() -> crate::gfx::sg_context_desc;
    }
}

pub mod imgui {
    //! `sokol_imgui.h`

    // suppress all warnings
    #![allow(warnings)]

    use crate::{app::*, gfx::*};

    include!("ffi/sokol_imgui.rs");
}
