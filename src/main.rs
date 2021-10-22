use ggez::event;
use ggez::ContextBuilder;

mod game;
use game::Blockade;

fn main() {
    run_ggez();
}

fn run_ggez() {
    let (mut ctx, event_loop) = ContextBuilder::new("Blockade", "Nikolai Steen Kjosnes")
        .build()
        .expect("aieee, could not create ggez context!");

    let my_game = Blockade::new(&mut ctx);

    event::run(ctx, event_loop, my_game);
}
