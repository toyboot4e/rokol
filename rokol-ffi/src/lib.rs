//! Rust FFI to Sokol modules

pub mod app {
    //! `sokol_app.h`

    // suppress all errors
    #![allow(warnings)]

    // Include generated bindings
    include!(concat!(env!("OUT_DIR"), "/sokol_app_ffi.rs"));
}
