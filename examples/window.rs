/*!
Just open a window with Rokol! See this [repository] for more practical examples.

[repository]: https://github.com/toyboot4e/rokol_learn_opengl
*/

use rokol::{app as ra, gfx as rg};

fn main() -> ra::glue::Result {
    let rokol = ra::glue::Rokol {
        w: 1280,
        h: 720,
        title: "Rokol - Window".to_string(),
        ..Default::default()
    };

    let mut app = AppData {};

    rokol.run(&mut app)
}

struct AppData {}

impl ra::RApp for AppData {
    fn init(&mut self) {
        println!("Hello, Rokol! And this is the `init` callback!");
        // NOTE: `Rokol` always discards the graphics so we have to set it up:
        rg::setup(&mut ra::glue::app_desc());
    }
}
