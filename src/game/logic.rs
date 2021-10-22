pub mod components;
use components::*;

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
            color: Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
            },
        };

        let p2 = Player {
            dir: Direction::Right,
            color: Color {
                r: 0.0,
                g: 0.0,
                b: 1.0,
            },
        };

        let p1_start = Position { x: 5, y: 5 };
        let p2_start = Position { x: w - 6, y: h - 5 };

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
            color: Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
            },
        };

        let p2 = Player {
            dir: Direction::Right,
            color: Color {
                r: 0.0,
                g: 0.0,
                b: 1.0,
            },
        };

        let p1_start = Position { x: 5, y: 5 };
        let p2_start = Position { x: w - 6, y: h - 5 };

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

    pub fn frame_advance(&mut self) -> bool {
        match Game::move_players(self) {
            GameResult::None => false,
            r => {
                self.gamelog.0.push(r);
                self.running = false;
                self.gameover = true;
                true
            }
        }
    }

    fn move_players(&mut self) -> GameResult {
        let result: GameResult;

        let (x1, y1) = (self.p1_pos.x, self.p1_pos.y);
        let p1: Player = *self.map[y1][x1].unwrap();

        let (x2, y2) = (self.p2_pos.x, self.p2_pos.y);
        let p2: Player = *self.map[y2][x2].unwrap();

        let p1_next = self
            .p1_pos
            .new_pos(p1.dir, (self.map[0].len(), self.map.len()));
        let p2_next = self
            .p2_pos
            .new_pos(p2.dir, (self.map[0].len(), self.map.len()));

        if p1_next == p2_next {
            result = GameResult::Draw;
        } else {
            match (
                self.map[p1_next.y][p1_next.x],
                self.map[p2_next.y][p2_next.x],
            ) {
                (Entity::Path(_) | Entity::Wall, Entity::None) => result = GameResult::P2Win,
                (Entity::None, Entity::Path(_) | Entity::Wall) => result = GameResult::P1Win,
                (Entity::Path(_) | Entity::Wall, Entity::Path(_) | Entity::Wall) => {
                    result = GameResult::Draw
                }
                (_, _) => result = GameResult::None,
            };
        }

        if result == GameResult::None {
            // Move p1
            self.map[p1_next.y][p1_next.x] = Entity::Player(p1);
            self.p1_pos = Position {
                x: p1_next.x,
                y: p1_next.y,
            };
            self.map[y1][x1] = Entity::Path(Path(p1.color));
            /*
            // Move p2
            self.map[p2_next.y][p2_next.x] = Entity::Player(p2);
            self.p2_pos = Position {
                x: p2_next.x,
                y: p2_next.y,
            };
            self.map[y2][x2] = Entity::Path(Path(p2.color));
            */
        }

        result
    }

    pub fn change_dir(&mut self, dir: Direction, player: usize) {
        if player == 1 {
            if !self.map[self.p1_pos.y][self.p1_pos.x]
                .unwrap()
                .dir
                .is_opposite(dir)
                || !self.running
            {
                self.map[self.p1_pos.y][self.p1_pos.x].unwrap().dir = dir;
            }
        } else {
            if !self.map[self.p2_pos.y][self.p2_pos.x]
                .unwrap()
                .dir
                .is_opposite(dir)
                || !self.running
            {
                self.map[self.p2_pos.y][self.p2_pos.x].unwrap().dir = dir;
            }
        }
    }
}
