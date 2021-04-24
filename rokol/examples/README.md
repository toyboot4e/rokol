# Examples

Run `sapp-clear` example with default features (`sokol_app.h` + `sokol_gfx.h`):

```sh
$ cargo run --example sapp-clear --features --impl-app,impl-gfx,glcore33
```

Run `sapp-clear` example with custom features (Rust-SDL2 + `sokol_gfx.h`):

```sh
$ cargo run --example sdl2-clear --features sdl2,impl-gfx,glcore33
```
