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
}

impl Engine<ChessMoveWrapper, ChessBoard> for ChessEngine {
    fn state(&self) -> ChessBoard {
        self.board.clone()
    }
}
