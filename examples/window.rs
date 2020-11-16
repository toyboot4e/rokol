//! Just open a window with Rokol!

fn main() -> rokol::Result {
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
    }
}
