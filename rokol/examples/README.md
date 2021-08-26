# Examples

Run `sapp-clear` example with default features (`sokol_app.h` + `sokol_gfx.h`):

```sh
$ cargo run --example sapp-clear --features impl-app,impl-gfx,glcore33
```

Run `sapp-clear` example with custom features (Rust-SDL2 + `sokol_gfx.h`):

```sh
$ cargo run --example sdl2-clear --features sdl2,impl-gfx,glcore33
```

More examples can be found in https://github.com/toyboot4e/rokol_learn_opengl[rokol_learn_opengl], but it's not for a while (since I'm focusing on my 2D game).

