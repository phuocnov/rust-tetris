use std::{fmt, ptr::null};

use crate::game_config::{PiecesColor, GRID_X_SIZE, GRID_Y_SIZE};
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
pub enum PieceTypes {
    SQUARE,
    LINE,
    T,
    L,
    J,
    Z,
    S,
}

impl fmt::Display for PieceTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PieceTypes::SQUARE => write!(f, "SQUARE"),
            PieceTypes::LINE => write!(f, "LINE"),
            PieceTypes::T => write!(f, "T"),
            PieceTypes::L => write!(f, "L"),
            PieceTypes::J => write!(f, "J"),
            PieceTypes::Z => write!(f, "Z"),
            PieceTypes::S => write!(f, "S"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Piece {
    pub piece_type: PieceTypes,
    pub position: Point,
    pub rotation: i32,
    pub color: PiecesColor,
    pub blocks: Vec<Point>,
}

impl Piece {
    pub fn new(&mut self, piece_type: PieceTypes) -> Piece {
        match piece_type {
            PieceTypes::SQUARE => Piece::new_square(),
            PieceTypes::LINE => Piece::new_line(),
            PieceTypes::T => Piece::new_T(),
            PieceTypes::L => Piece::new_L(),
            PieceTypes::J => Piece::new_J(),
            PieceTypes::Z => Piece::new_Z(),
            PieceTypes::S => Piece::new_S(),
        }
    }

    fn new_square() -> Piece {
        Piece {
            piece_type: PieceTypes::SQUARE,
            position: Point(0, 0),
            rotation: 0,
            color: PiecesColor::YELLOW,
            blocks: vec![Point(0, 0), Point(1, 0), Point(0, 1), Point(1, 1)],
        }
    }

    fn new_line() -> Piece {
        Piece {
            piece_type: PieceTypes::LINE,
            position: Point(0, 0),
            rotation: 0,
            color: PiecesColor::CYAN,
            blocks: vec![Point(0, 0), Point(1, 0), Point(2, 0), Point(3, 0)],
        }
    }

    fn new_T() -> Piece {
        Piece {
            piece_type: PieceTypes::T,
            position: Point(0, 0),
            rotation: 0,
            color: PiecesColor::PURPLE,
            blocks: vec![Point(0, 0), Point(1, 0), Point(2, 0), Point(1, 1)],
        }
    }

    fn new_L() -> Piece {
        Piece {
            piece_type: PieceTypes::L,
            position: Point(0, 0),
            rotation: 0,
            color: PiecesColor::ORANGE,
            blocks: vec![Point(0, 0), Point(1, 0), Point(2, 0), Point(2, 1)],
        }
    }

    fn new_J() -> Piece {
        Piece {
            piece_type: PieceTypes::J,
            position: Point(0, 0),
            rotation: 0,
            color: PiecesColor::BLUE,
            blocks: vec![Point(0, 0), Point(1, 0), Point(2, 0), Point(0, 1)],
        }
    }

    fn new_Z() -> Piece {
        Piece {
            piece_type: PieceTypes::Z,
            position: Point(0, 0),
            rotation: 0,
            color: PiecesColor::RED,
            blocks: vec![Point(0, 0), Point(1, 0), Point(1, 1), Point(2, 1)],
        }
    }

    fn new_S() -> Piece {
        Piece {
            piece_type: PieceTypes::S,
            position: Point(0, 0),
            rotation: 0,
            color: PiecesColor::GREEN,
            blocks: vec![Point(0, 1), Point(1, 1), Point(1, 0), Point(2, 0)],
        }
    }
}

#[derive(Clone, Debug)]
pub struct GameContext {
    pub current_piece: Piece,
    pub next_piece: Piece,
    pub holding_piece: Piece,
    pub state: GameState,
}

impl GameContext {
    pub fn new() -> GameContext {
        GameContext {
            state: GameState::Paused,
            current_piece: Piece::new_J(),
            next_piece: Piece::new_square(),
            holding_piece: Piece::new_square(),
        }
    }
}

// impl fmt::Display for GameContext {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "GameContext {{\n\tstate: {},\n\tplayer_position: {:?},\n\tfood: {}\n}}",
//             self.state, self.player_position, self.food
//         )
//     }
// }
