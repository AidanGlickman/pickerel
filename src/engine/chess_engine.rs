use crate::engine::engine::Engine;
use crate::state::board::{ChessBoard, ChessMoveWrapper};
use crate::state::state::{Move, State, StateEval};

pub struct ChessEngine {
    pub board: ChessBoard,
}

impl ChessEngine {
    pub fn new() -> Self {
        ChessEngine {
            board: ChessBoard::default(),
        }
    }
}

impl Engine<ChessMoveWrapper, ChessBoard> for ChessEngine {
    fn legal_moves(&self) -> Vec<ChessMoveWrapper> {
        self.board.legal_moves()
    }

    fn score(&self, evaluator: &dyn StateEval<ChessMoveWrapper, ChessBoard>) -> f64 {
        evaluator.evaluate(&self.board)
    }

    fn make_move(&self, mov: ChessMoveWrapper) {
        self.board.make_move(mov);
    }

    // fn state(&self) -> &ChessBoard {
    //     &self.board
    // }
}
