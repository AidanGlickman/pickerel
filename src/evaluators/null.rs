use crate::state::board::{ChessBoard, ChessMoveWrapper};
use crate::state::state::StateEval;

// Not at all useful
pub struct NullEval;

impl StateEval<ChessMoveWrapper, ChessBoard> for NullEval {
    fn evaluate(&self, board: &ChessBoard) -> f64 {
        0.0
    }
}
