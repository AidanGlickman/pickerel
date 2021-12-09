use crate::state::board::{ChessBoard, ChessMoveWrapper};
use crate::state::state::StateEval;

// Not at all useful
pub struct NullEval;

impl StateEval<ChessMoveWrapper, ChessBoard> for NullEval {
    fn evaluate(&self, _board: &ChessBoard) -> f64 {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::engine::Engine;
    use crate::evaluators::null::NullEval;

    #[test]
    fn null_eval_always_zero_minimax_naive() {
        let engine = crate::engine::chess_engine::ChessEngine::new();
        assert!(engine.minimax_naive(&NullEval, 0, true) == 0.0);
        assert!(engine.minimax_naive(&NullEval, 1, true) == 0.0);
        assert!(engine.minimax_naive(&NullEval, 2, true) == 0.0);
    }

    #[test]
    fn null_eval_always_zero_abdada() {
        let engine = crate::engine::chess_engine::ChessEngine::new();
        assert!(engine.abdada(&NullEval, 1) == 0.0);
        assert!(engine.abdada(&NullEval, 2) == 0.0);
    }
}
