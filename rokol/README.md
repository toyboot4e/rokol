# Rokol

Bindings to [Sokol](https://github.com/floooh/sokol) for personal use. [API](https://docs.rs/rokol/latest/rokol/)

Very early in progress..

## About

### Status

* Rokol only cares about desktop platforms.
* Rokol only supports GlCore33 backend (for now).
* **Rokol is tested on macOS only**. You could find it doesn't compile out of the box. Please open an issue then!

### Features

Enable features:

* `impl-app`: Compile `sokol_app.h`
* `sdl2`: Use SDL2
* `impl-gfx`: Compile `sokol_gfx.h`. Specify graphics backend with feature:
    * `glcore33`: compile `sokol_gfx.h` with GlCore33 backend
* `fontstash`: Add rokol graphics support for FontStash

## Notes

My devlog is [here](https://github.com/toyboot4e/rokol/blob/master/devlog.adoc).

