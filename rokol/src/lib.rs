/*!
Wrapper of [Sokol] libraries

[Sokol]: https://github.com/floooh/sokol

# Features (specified in `Cargo.toml`)

Example settings:

```toml
rokol = { features = ["sdl", "impl-gfx", "glcore33", "fontstash"] }
```

* `impl-app`: implements `sokol_app.h` and enables `app` module
* `sdl2`: generates [`glue`] code for `sdl2`
* `impl-gfx`: implements `sokol_gfx.h` and enables `gfx` module
  * `glcore33`: uses OpenGL backend
  * `metal`: uses Metal backend
  * `d3d11`: uses DirectX11 backend
* `fontstash`: implements `fontstash.h` and enables `fons` module

# Tips

* Checkout [The Brain Dump]
  * Sokol [considers] zero-initizialized structures to be in default state. It means
    [`Default::default`] is ensured to make sense!
* use `bytemuck` to cast types to `&[u8]`.

[The Brain Dump]: https://floooh.github.io/
[considers]: https://floooh.github.io/2017/08/06/sokol-api-update.html
*/

pub use rokol_ffi as ffi;

/// Creates an `enum` from FFI enum type (output of bindgen as a rustified enum)
macro_rules! ffi_enum {
    (
        $(#[$outer:meta])*
        $vis:vis enum $Enum:ident around $Ffi:ty {
            $(
                $(#[$attr:ident $($args:tt)*])*
                $variant:ident = $ffi_variant:ident,
            )*
        }

        $($t:tt)*
    ) => {
        $(#[$outer])*
        #[repr(u32)]
        $vis enum $Enum {
            $(
                $(#[$attr $($args)*])*
                $variant = <$Ffi>::$ffi_variant as u32,
            )*
        }

        impl $Enum {
            pub fn from_ffi(ffi_variant: $Ffi) -> Self {
                match ffi_variant {
                    $(
                        <$Ffi>::$ffi_variant => Self::$variant,
                    )*
                    _ => panic!("Bug: not convered FFI enum!"),
                }
            }

            pub fn to_ffi(self) -> $Ffi {
                match self {
                    $(
                        <Self>::$variant => <$Ffi>::$ffi_variant,
                    )*
                }
            }
        }

        impl From<$Ffi> for $Enum {
            fn from(ffi_variant: $Ffi) -> Self {
                Self::from_ffi(ffi_variant)
            }
        }

        impl Into<$Ffi> for $Enum {
            fn into(self) -> $Ffi {
                Self::to_ffi(self)
            }
        }
    };
}

#[cfg(feature = "impl-app")]
pub mod app;

#[cfg(feature = "impl-gfx")]
pub mod gfx;

#[cfg(feature = "impl-gfx")]
pub mod glue;

#[cfg(all(feature = "impl-gfx", feature = "fontstash"))]
pub mod fons;
