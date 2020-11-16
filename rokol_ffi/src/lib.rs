//! Rust FFI to Sokol modules

pub mod app {
    //! `sokol_app.h`

    // suppress all errors
    #![allow(warnings)]

    // Include generated bindings
    include!(concat!(env!("OUT_DIR"), "/sokol_app_ffi.rs"));
}

pub mod gfx {
    //! `sokol_gfx.h`

    // suppress all errors
    #![allow(warnings)]

    // Include generated bindings
    include!(concat!(env!("OUT_DIR"), "/sokol_gfx_ffi.rs"));
}

#[cfg(test)]
mod test {
    #[test]
    fn link_test() {
        let _desc = unsafe { crate::app::sokol_main(0, std::ptr::null_mut()) };
    }
}
