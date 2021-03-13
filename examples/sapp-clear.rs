/*!
$ cargo run --features impl-app,impl-gfx,glcore33 --example sapp-clear
*/

use rokol::{app as ra, gfx as rg, glue::sapp as glue};

fn main() -> glue::Result {
    let rokol = glue::Rokol {
        w: 1280,
        h: 720,
        title: "Rokol - Clear".to_string(),
        ..Default::default()
    };

    let mut app = AppData::new();

    rokol.run(&mut app)
}

#[derive(Debug)]
struct AppData {
    /// Clears the frame color buffer on starting screen rendering pass
    pa: rg::PassAction,
}

impl AppData {
    pub fn new() -> Self {
        let pa = rg::PassAction::clear([100.0 / 255.0, 149.0 / 255.0, 237.0 / 255.0, 1.0]);

        Self { pa }
    }
}

impl ra::RApp for AppData {
    fn init(&mut self) {
        rg::setup(&mut glue::app_desc());
    }

    fn frame(&mut self) {
        // start rendering pass to the frame buffer
        rg::begin_default_pass(&self.pa, ra::width(), ra::height());
        rg::end_pass();
        rg::commit();
    }
}
