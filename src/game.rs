// ggez
use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics::{self, Color};
use ggez::{timer, Context, GameResult};

const FPS: u32 = 8;
const SQ_SIZE: usize = 26;

mod logic;
use logic::components::{Direction, Entity};
use logic::Game;

pub struct Blockade {
    g: Game,
}

impl Blockade {
    pub fn new(_ctx: &mut Context) -> Blockade {
        Blockade { g: Game::new() }
    }
}

impl Entity {
    fn draw(&self, ctx: &mut Context, x: usize, y: usize) -> GameResult<()> {
        let color;

        match self {
            Entity::Player(p) => color = [p.color.r, p.color.g, p.color.b, 1.0].into(),
            Entity::Path(p) => color = [p.0.r, p.0.g, p.0.b, 1.0].into(),
            Entity::Wall => color = [1.0, 1.0, 1.0, 1.0].into(),
            Entity::None => color = [0.5, 0.5, 0.5, 1.0].into(),
        }

        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            ggez::graphics::Rect::new(
                (x * SQ_SIZE) as f32,
                (y * SQ_SIZE) as f32,
                SQ_SIZE as f32 - 2.0,
                SQ_SIZE as f32 - 2.0,
            ),
            color,
        )?;
        graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))
    }
}

impl logic::components::Log {
    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let font = graphics::Font::new(ctx, "/LiberationMono-Regular.ttf")?;
        let score = self.get_score();
        let scoreboard = format!("P1:   {}\nDraw: {}\nP2:   {}", score[0], score[1], score[2]);
        let txt = graphics::Text::new((scoreboard, font, 48.0));
        graphics::draw(
            ctx,
            &txt,
            (
                ggez::mint::Point2 { x: 575.0, y: 50.0 },
                graphics::Color::BLACK,
            ),
        )?;
        Ok(())
    }
}

impl EventHandler<ggez::GameError> for Blockade {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while timer::check_update_time(ctx, FPS) {
            if self.g.running {
                self.g.frame_advance();
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::WHITE);

        for y in 0..self.g.map.len() {
            for x in 0..self.g.map[0].len() {
                self.g.map[y][x].draw(ctx, x, y)?
            }
        }

        self.g.gamelog.draw(ctx)?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Up => self.g.change_dir(Direction::Up, 1),
            KeyCode::Down => self.g.change_dir(Direction::Down, 1),
            KeyCode::Left => self.g.change_dir(Direction::Left, 1),
            KeyCode::Right => self.g.change_dir(Direction::Right, 1),
            KeyCode::W => self.g.change_dir(Direction::Up, 2),
            KeyCode::S => self.g.change_dir(Direction::Down, 2),
            KeyCode::A => self.g.change_dir(Direction::Left, 2),
            KeyCode::D => self.g.change_dir(Direction::Right, 2),
            KeyCode::R => self.g.reset(),
            KeyCode::P => self.g.pause(2),
            _ => {}
        }
    }
}
