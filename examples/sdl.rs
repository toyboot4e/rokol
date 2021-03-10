//! $ cargo run --features glcore33,use-sdl2

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use rokol::gfx as rg;

fn main() -> Result<()> {
    let handle = rokol::sdl::Init {
        name: "Rokol + Rust-SDL2".to_string(),
        w: 1280,
        h: 720,
        settings: Default::default(),
    }
    .init(|window_builder| {
        window_builder.position_centered();
    })?;

    // clear screen with cornflower blue
    let pa = rg::PassAction::clear([100.0 / 255.0, 149.0 / 255.0, 237.0 / 255.0, 1.0]);

    let mut event_pump = handle.sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        rg::begin_default_pass(&pa, 1280, 720);
        rg::end_pass();
        rg::commit();
        handle.swap_window();

        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    rg::shutdown();

    Ok(())
}
