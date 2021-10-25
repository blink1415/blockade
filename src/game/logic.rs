pub mod components;
use components::*;

const COLOR_MULTIPLIER: f32 = 0.6;

pub struct Game {
    pub map: [[Entity; 22]; 22],
    pub gamelog: Log,
    p1_pos: Position,
    p2_pos: Position,
    pub running: bool,
    pub gameover: bool,
}

impl Game {
    pub fn new() -> Game {
        // Initialize game map

        let mut map = [[Entity::None; 22]; 22];

        let (w, h) = (map[0].len(), map.len());

        for i in 0..h {
            map[0][i] = Entity::Wall;
            map[w - 1][i] = Entity::Wall;
            map[i][0] = Entity::Wall;
            map[i][h - 1] = Entity::Wall;
        }

        // Place players

        let p1 = Player {
            dir: Direction::Left,
            last_dir: Direction::Left,
            color: Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
            },
            paused: false,
        };

        let p2 = Player {
            dir: Direction::Right,
            last_dir: Direction::Right,
            color: Color {
                r: 0.0,
                g: 0.0,
                b: 1.0,
            },
            paused: false,
        };

        let p1_start = Position {
            x: 7 * w / 8,
            y: h / 2,
        };
        let p2_start = Position { x: w / 8, y: h / 2 };

        map[p1_start.y][p1_start.x] = Entity::Player(p1);
        map[p2_start.y][p2_start.x] = Entity::Player(p2);

        Game {
            map: map,
            gamelog: Log(Vec::new()),
            p1_pos: p1_start,
            p2_pos: p2_start,
            running: false,
            gameover: false,
        }
    }

    pub fn reset(&mut self) {
        let mut map = [[Entity::None; 22]; 22];

        let (w, h) = (map[0].len(), map.len());

        for i in 0..h {
            map[0][i] = Entity::Wall;
            map[w - 1][i] = Entity::Wall;
            map[i][0] = Entity::Wall;
            map[i][h - 1] = Entity::Wall;
        }

        let p1 = Player {
            dir: Direction::Left,
            last_dir: Direction::Left,
            color: Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
            },
            paused: false,
        };

        let p2 = Player {
            dir: Direction::Right,
            last_dir: Direction::Right,
            color: Color {
                r: 0.0,
                g: 0.0,
                b: 1.0,
            },
            paused: false,
        };

        let p1_start = Position {
            x: 7 * w / 8,
            y: h / 2,
        };
        let p2_start = Position { x: w / 8, y: h / 2 };

        map[p1_start.y][p1_start.x] = Entity::Player(p1);
        map[p2_start.y][p2_start.x] = Entity::Player(p2);

        self.p1_pos = p1_start;
        self.p2_pos = p2_start;

        self.map = map;

        self.running = false;
        self.gameover = false;
    }

    // print function for debug purposes
    #[allow(dead_code)]
    pub fn print_map(&self) -> String {
        let mut out = String::new();
        for y in 0..self.map.len() {
            for x in 0..self.map[0].len() {
                match self.map[y][x] {
                    Entity::Player(_) => out += "P",
                    Entity::Path(_) => out += "#",
                    Entity::Wall => out += "|",
                    Entity::None => out += ".",
                }
            }
            out += "\n";
        }

        out
    }

    pub fn frame_advance(&mut self) {
        let (p1_next, p2_next) = self.get_next_positions();

        match self.get_result(p1_next, p2_next) {
            GameResult::None => { /* next frame */ }
            r => {
                self.gamelog.0.push(r);
                self.running = false;
                self.gameover = true;
            }
        };

        Game::move_players(self, p1_next, p2_next);
    }

    fn get_next_positions(&mut self) -> (Position, Position) {
        let (x1, y1) = (self.p1_pos.x, self.p1_pos.y);
        let (x2, y2) = (self.p2_pos.x, self.p2_pos.y);

        let mut p1_next = Position { x: 0, y: 0 };
        let mut p2_next = Position { x: 0, y: 0 };

        if let Entity::Player(p1) = self.map[y1][x1] {
            p1_next = self
                .p1_pos
                .new_pos(p1.dir, (self.map[0].len(), self.map.len()));
        }
        if let Entity::Player(p2) = self.map[y2][x2] {
            p2_next = self
                .p2_pos
                .new_pos(p2.dir, (self.map[0].len(), self.map.len()));
        }

        (p1_next, p2_next)
    }

    fn move_players(&mut self, p1_next: Position, p2_next: Position) {
        let (x1, y1) = (self.p1_pos.x, self.p1_pos.y);
        let (x2, y2) = (self.p2_pos.x, self.p2_pos.y);

        self.map[y1][x1].update_dir();
        self.map[y2][x2].update_dir();

        match (self.map[y1][x1], self.map[y2][x2]) {
            (Entity::Player(p1), Entity::Player(p2)) => {
                if !p1.paused {
                    self.map[p1_next.y][p1_next.x] = Entity::Player(p1);
                    self.p1_pos = Position {
                        x: p1_next.x,
                        y: p1_next.y,
                    };
                    let path_color = Color {
                        r: p1.color.r * COLOR_MULTIPLIER,
                        g: p1.color.g * COLOR_MULTIPLIER,
                        b: p1.color.b * COLOR_MULTIPLIER,
                    };
                    self.map[y1][x1] = Entity::Path(Path(path_color));
                }

                if !p2.paused {
                    self.map[p2_next.y][p2_next.x] = Entity::Player(p2);
                    self.p2_pos = Position {
                        x: p2_next.x,
                        y: p2_next.y,
                    };
                    let path_color = Color {
                        r: p2.color.r * COLOR_MULTIPLIER,
                        g: p2.color.g * COLOR_MULTIPLIER,
                        b: p2.color.b * COLOR_MULTIPLIER,
                    };
                    self.map[y2][x2] = Entity::Path(Path(path_color));
                }
            }
            (_, _) => panic!("Unreachable code!"),
        }
    }

    fn get_result(&self, p1_next: Position, p2_next: Position) -> GameResult {
        let p1_dead = match self.map[p1_next.y][p1_next.x] {
            Entity::None => false,
            _ => true,
        };

        let p2_dead = match self.map[p2_next.y][p2_next.x] {
            Entity::None => false,
            _ => true,
        };

        match (p1_dead, p2_dead) {
            (true, true) => GameResult::Draw,
            (false, true) => GameResult::P1Win,
            (true, false) => GameResult::P2Win,
            (false, false) => GameResult::None,
        }
    }

    pub fn change_dir(&mut self, dir: Direction, player: usize) {
        if player == 1 {
            if !self.map[self.p1_pos.y][self.p1_pos.x]
                .copy()
                .last_dir
                .is_opposite(dir)
                || !self.running
            {
                self.map[self.p1_pos.y][self.p1_pos.x].copy().dir = dir;
            }
        } else {
            if !self.map[self.p2_pos.y][self.p2_pos.x]
                .copy()
                .last_dir
                .is_opposite(dir)
                || !self.running
            {
                self.map[self.p2_pos.y][self.p2_pos.x].copy().dir = dir;
            }
        }

        if !self.running && !self.gameover {
            self.running = true;
        }
    }

    pub fn pause(&mut self, player: usize) {
        if player == 1 {
            match &mut self.map[self.p1_pos.y][self.p1_pos.x] {
                Entity::Player(p) => p.paused = !p.paused,
                _ => panic!("Unreachable code!"),
            }
        } else if player == 2 {
            match &mut self.map[self.p2_pos.y][self.p2_pos.x] {
                Entity::Player(p) => p.paused = !p.paused,
                _ => panic!("Unreachable code!"),
            }
        }
    }
}
