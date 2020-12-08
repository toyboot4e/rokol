/*!

Rust FFI to [Sokol] headers for [Rokol]

[Sokol]: https://github.com/floooh/sokol
[Rokol]: https://github.com/toyboot4e/rokol


Generated with [`bindgen`], implementing [`Default`] trait
([`Bindgen::derive_default(true)`][derive]).

[`bindgen`]: https://docs.rs/bindgen/latest/bindgen
[derive]: https://docs.rs/bindgen/0.56.0/bindgen/struct.Builder.html#method.derive_default

Last update: Dec 3, 2020 ([commit]). Sokol header declaration diffs can be seen on [GitHub][Rokol].

[commit]: https://github.com/floooh/sokol/commit/64a6f2e2fac607ddcd4ccc5bd8bcd25946293550

*/

pub mod app {
    //! FFI to [`sokol_app.h`](https://github.com/floooh/sokol/blob/master/sokol_app.h)

    // suppress all warnings
    #![allow(warnings)]

    // Include generated bindings
    include!("ffi/sokol_app.rs");
}

pub mod gfx {
    //! FFI to [`sokol_gfx.h`](https://github.com/floooh/sokol/blob/master/sokol_gfx.h)

    // suppress all warnings
    #![allow(warnings)]

    include!("ffi/sokol_gfx.rs");
}

pub mod glue {
    //! FFI to [`sokol_glue.h`](https://github.com/floooh/sokol/blob/master/sokol_glue.h)

    // there's only one function so let's write it manually
    extern "C" {
        pub fn sapp_sgcontext() -> crate::gfx::sg_context_desc;
    }
}

// pub mod imgui {
//     //! `sokol_imgui.h`, `sokol_gfx_imgui.h`
//
//     // suppress all warnings
//     #![allow(warnings)]
//
//     // blacklisted items
//     use crate::{app::*, gfx::*};
//
//     include!("ffi/sokol_imgui.rs");
// }
