use std::fmt;

use crate::game_config::{GRID_X_SIZE, GRID_Y_SIZE};
use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameState {
    Playing,
    Paused,
    Menu,
    GameOver,
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GameState::Playing => write!(f, "Playing"),
            GameState::Paused => write!(f, "Paused"),
            GameState::Menu => write!(f, "Menu"),
            GameState::GameOver => write!(f, "GameOver"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Point(pub i32, pub i32);

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Pieces {
    SQUARE,
    LINE,
    T,
    L,
    Z,
    S,
}

impl fmt::Display for Pieces {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Pieces::SQUARE => write!(f, "SQUARE"),
            Pieces::LINE => write!(f, "LINE"),
            Pieces::T => write!(f, "T"),
            Pieces::L => write!(f, "L"),
            Pieces::Z => write!(f, "Z"),
            Pieces::S => write!(f, "S"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct GameContext {
    pub player_position: Vec<Point>,
    pub food: Point,
    pub state: GameState,
}

impl GameContext {
    pub fn new() -> GameContext {
        GameContext {
            player_position: vec![Point(3, 1), Point(2, 1), Point(1, 1)],
            state: GameState::Paused,
            food: Point(3, 3),
        }
    }
}

impl fmt::Display for GameContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "GameContext {{\n\tstate: {},\n\tplayer_position: {:?},\n\tfood: {}\n}}",
            self.state, self.player_position, self.food
        )
    }
}
