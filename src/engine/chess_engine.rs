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

    fn legal_moves(&self) -> Vec<ChessMoveWrapper> {
        self.board.legal_moves()
    }

    fn score(&self, evaluator: &dyn StateEval<ChessMoveWrapper, ChessBoard>) -> f64 {
        evaluator.evaluate(&self.board)
    }

    // fn make_move(&self, mov: ChessMoveWrapper) -> ChessEngine {
    //     ChessEngine {
    //         board: self.board.make_move(mov),
    //         cache: self.cache.clone(),
    //     }
    // }

    fn add_to_cache(&mut self, state: ChessBoard, score_depth: (f64, usize)) {
        match self.cache.get(&state) {
            Some(score_depth_found) => {
                if score_depth_found.1 < score_depth.1 {
                    self.cache.insert(state, score_depth);
                }
            }
            None => {
                self.cache.insert(state, score_depth);
            }
        }
    }

    fn get_from_cache(&self, state: &ChessBoard) -> Option<(f64, usize)> {
        match self.cache.get(&state) {
            Some(score_depth) => Some(score_depth.clone()),
            None => None,
        }
    }

    // fn state(&self) -> &ChessBoard {
    //     &self.board
    // }
}
