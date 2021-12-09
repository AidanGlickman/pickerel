use crate::engine::engine::Engine;
use crate::state::board::{ChessBoard, ChessMoveWrapper};
use crate::state::state::{State, StateEval};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ChessEngine {
    pub board: ChessBoard,
    pub cache: HashMap<ChessBoard, (f64, usize)>,
}

impl ChessEngine {
    pub fn new() -> Self {
        ChessEngine {
            board: ChessBoard::default(),
            cache: HashMap::new(),
        }
    }

    pub fn from_str(fen: String) -> Self {
        ChessEngine {
            board: ChessBoard::from_str(fen),
            cache: HashMap::new()
        }
    }

    pub fn update_fen(&mut self, fen: String) {
        self.board = ChessBoard::from_str(fen);
    }
}

impl Engine<ChessMoveWrapper, ChessBoard> for ChessEngine {
    fn state(&self) -> ChessBoard {
        self.board.clone()
    }
}
