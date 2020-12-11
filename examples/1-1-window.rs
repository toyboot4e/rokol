//! Just open a window with Rokol!

use rokol::gfx as rg;

fn main() -> rokol::Result {
    env_logger::init(); // give implementation to log crate

    let rokol = rokol::Rokol {
        w: 1280,
        h: 720,
        title: "Rokol - Window".to_string(),
        ..Default::default()
    };

    let mut app = AppData {};

    rokol.run(&mut app)
}

struct AppData {}

impl rokol::app::RApp for AppData {
    fn init(&mut self) {
        println!("Hello, Rokol! And this is the `init` callback!");
        // NOTE: `Rokol` always discards the graphics so we have to set it up:
        rg::setup(&mut rokol::glue::app_desc());
    }
}
