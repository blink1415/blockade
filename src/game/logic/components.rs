// Part-structs

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    // Gets a new position moved 1 space to the given direction, avoiding going out of specified bounds
    pub fn new_pos(&mut self, dir: Direction, limits: (usize, usize)) -> Position {
        match dir {
            Direction::Up => Position {
                x: self.x,
                y: if self.y > 0 { self.y - 1 } else { 0 },
            },
            Direction::Down => Position {
                x: self.x,
                y: if self.y < limits.1 {
                    self.y + 1
                } else {
                    limits.1
                },
            },
            Direction::Left => Position {
                x: if self.x > 0 { self.x - 1 } else { 0 },
                y: self.y,
            },
            Direction::Right => Position {
                x: if self.x < limits.0 {
                    self.x + 1
                } else {
                    limits.0
                },
                y: self.y,
            },
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn is_opposite(&self, dir: Direction) -> bool {
        match (self, dir) {
            (Direction::Up, Direction::Down)
            | (Direction::Down, Direction::Up)
            | (Direction::Left, Direction::Right)
            | (Direction::Right, Direction::Left) => true,
            _ => false,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[derive(Debug)]
pub struct Log(pub Vec<GameResult>);

impl Log {
    pub fn get_score(&self) -> [usize; 3] {
        let mut score = [0; 3];

        for s in &self.0 {
            match s {
                GameResult::P1Win => score[0] += 1,
                GameResult::P2Win => score[2] += 1,
                GameResult::Draw => score[1] += 1,
                GameResult::None => {
                    panic!("Game result 'None' added to gamelog. This isn't supposed to happen.")
                }
            }
        }

        score
    }
}

// Composite-structs

#[derive(Copy, Clone, Debug)]
pub struct Player {
    pub dir: Direction,
    pub last_dir: Direction,
    pub color: Color,
    pub paused: bool,
}

#[derive(Copy, Clone, Debug)]
pub struct Path(pub Color);

#[derive(Copy, Clone, Debug)]
pub enum Entity {
    Player(Player),
    Path(Path),
    Wall,
    None,
}

impl Entity {
    pub fn copy(&mut self) -> &mut Player {
        match self {
            Entity::Player(p) => p,
            _ => panic!(
                "Attempted to unwrap a non-player entity! Details: {:?}",
                self
            ),
        }
    }

    pub fn update_dir(&mut self) {
        match self {
            Entity::Player(p) => p.last_dir = p.dir,
            _ => {}
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum GameResult {
    P1Win,
    P2Win,
    Draw,
    None,
}
