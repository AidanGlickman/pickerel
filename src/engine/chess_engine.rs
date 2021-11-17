use crate::engine::engine::Engine;
use crate::state::board::{ChessBoard, ChessMoveWrapper};
use crate::state::state::{Move, State, StateEval};

#[derive(Clone, Debug)]
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
    fn state(&self) -> ChessBoard {
        self.board.clone()
    }

    fn legal_moves(&self) -> Vec<ChessMoveWrapper> {
        self.board.legal_moves()
    }

    fn score(&self, evaluator: &dyn StateEval<ChessMoveWrapper, ChessBoard>) -> f64 {
        evaluator.evaluate(&self.board)
    }

    fn make_move(&self, mov: ChessMoveWrapper) -> ChessEngine {
        ChessEngine {
            board: self.board.make_move(mov),
        }
    }

    // fn state(&self) -> &ChessBoard {
    //     &self.board
    // }
}
