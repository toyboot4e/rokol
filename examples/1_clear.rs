//! Open a window and fill it with corn-flower blue!

use rokol::{app as ra, gfx as rg};

fn main() -> rokol::Result {
    env_logger::init(); // give implementation to log crate

    let rokol = rokol::Rokol {
        w: 1280,
        h: 720,
        title: "Rokol - Window".to_string(),
        ..Default::default()
    };

    let mut app = AppData::new();

    rokol.run(&mut app)
}

#[derive(Debug)]
struct AppData {
    pa: rg::PassAction,
}

impl AppData {
    pub fn new() -> Self {
        let pa = rg::PassAction::clear([100.0 / 255.0, 149.0 / 255.0, 237.0 / 255.0, 1.0]);

        Self { pa }
    }
}

impl rokol::app::RApp for AppData {
    fn init(&mut self) {
        rg::setup(&mut rokol::app_desc());
    }

    fn frame(&mut self) {
        rg::begin_default_pass(&self.pa, ra::width(), ra::height());
        rg::end_pass();
        rg::commit();
    }
}
