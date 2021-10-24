use ggez::event;
use ggez::ContextBuilder;

use std::{env, path};

mod game;
use game::{Blockade, SQ_SIZE};

fn main() {
    run_ggez();
}

fn run_ggez() {
    // Resource path
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (mut ctx, event_loop) = ContextBuilder::new("Blockade", "Nikolai Steen Kjosnes")
        .window_setup(ggez::conf::WindowSetup::default().title("Blockade"))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions((SQ_SIZE * 30) as f32, (SQ_SIZE * 22 + 10) as f32),
        )
        .add_resource_path(resource_dir)
        .build()
        .expect("An error occurred while creating ggez context.");

    let my_game = Blockade::new(&mut ctx);

    event::run(ctx, event_loop, my_game);
}
