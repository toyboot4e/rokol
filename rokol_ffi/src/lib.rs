/*!
Rust FFI to [Sokol] headers for [Rokol] ([API](https://docs.rs/rokol/latest/rokol/))

Last update: Dec 3, 2020 ([commit]). Sokol header declaration diffs can be seen on [GitHub][Rokol].

[Sokol]: https://github.com/floooh/sokol
[Rokol]: https://github.com/toyboot4e/rokol
[commit]: https://github.com/floooh/sokol/commit/5fbb6a25501e5478674ca0600a7539033592d749

# impl Default

Generated with [`bindgen`], implementing [`Default`] trait
([`Bindgen::derive_default(true)`][derive]).

NOTE: Sokol [considers] zero-initizialized structures to be in default state. It means
[`Default::default`] is ensured to make sense!

[`bindgen`]: https://docs.rs/bindgen/latest/bindgen
[derive]: https://docs.rs/bindgen/0.56.0/bindgen/struct.Builder.html#method.derive_default
[considers]: https://floooh.github.io/2017/08/06/sokol-api-update.html
*/

// TODO: Do not use `include!` so that we get goto support in Emacs
// https://docs.rs/bindgen/latest/bindgen/struct.Builder.html#method.module_raw_lines

#[cfg(feature = "impl-app")]
pub mod app;

#[cfg(feature = "impl-gfx")]
pub mod gfx;

#[cfg(all(feature = "impl-app", feature = "impl-gfx"))]
pub mod glue {
    //! FFI to [`sokol_glue.h`](https://github.com/floooh/sokol/blob/master/sokol_glue.h)

    // there's only one function so let's write it manually
    extern "C" {
        pub fn sapp_sgcontext() -> crate::gfx::sg_context_desc;
    }
}

// #[cfg(test)]
// mod test {
//     /// Just to make sure we link to `sokol`
//     fn link_test() {
//         unsafe {
//             let mut desc: crate::gfx::sg_desc = Default::default();
//             desc.context = crate::glue::sapp_sgcontext();
//             crate::gfx::sg_setup(&desc);
//         }
//     }
// }

#[cfg(feature = "impl-gfx")]
mod gfx_impls {
    use super::*;
    use gfx::sg_color;

    impl From<[f32; 4]> for sg_color {
        fn from(xs: [f32; 4]) -> sg_color {
            sg_color {
                r: xs[0],
                g: xs[1],
                b: xs[2],
                a: xs[3],
            }
        }
    }

    impl From<[&f32; 4]> for sg_color {
        fn from(xs: [&f32; 4]) -> sg_color {
            sg_color {
                r: *xs[0],
                g: *xs[1],
                b: *xs[2],
                a: *xs[3],
            }
        }
    }

    impl From<&[u8]> for gfx::sg_range {
        // WARNING: This is VERY UNSAFE since the slice may be a temporary value
        fn from(x: &[u8]) -> gfx::sg_range {
            gfx::sg_range {
                ptr: x.as_ptr() as *const _,
                size: x.len() as _,
            }
        }
    }
}
