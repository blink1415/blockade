use ggez::event;
use ggez::ContextBuilder;

use std::{env, path};

mod game;
use game::Blockade;

fn main() {
    run_ggez();
}

fn run_ggez() {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (mut ctx, event_loop) = ContextBuilder::new("Blockade", "Nikolai Steen Kjosnes")
        .add_resource_path(resource_dir)
        .build()
        .expect("An error occurred while creating ggez context.");

    let my_game = Blockade::new(&mut ctx);

    event::run(ctx, event_loop, my_game);
}
