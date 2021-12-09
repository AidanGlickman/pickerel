use crate::state::board::{ChessBoard, ChessMoveWrapper};
use crate::state::state::{State, StateEval};
use crate::evaluators::end;

pub struct NaiveEval;

pub const weights: [usize; 6] = [
    1,
    3,
    3,
    5,
    9,
    10000
];

impl StateEval<ChessMoveWrapper, ChessBoard> for NaiveEval {
    fn evaluate(&self, _board: &ChessBoard) -> f64 {
        if let Some(value) = end::check_end_pos(_board) {
            value;
        } 

        let mut value: i32 = 0;
        for color in chess::ALL_COLORS {
            let mut i: usize = 0;
            for piece in chess::ALL_PIECES {
                let mut sign = 1;

                if color == chess::Color::Black {
                    sign = -1;
                }

                value += sign * (((_board.board.color_combined(color) & _board.board.pieces(piece)).popcnt()* (weights[i] as u32)) as i32);
                i +=1;
            }
        }
        return (value as f64);
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::engine::Engine;
    use crate::evaluators::naive::NaiveEval;

    #[test]
    fn naive_eval_white_winning() {
        let engine = crate::engine::chess_engine::ChessEngine::from_str("Q5nr/p2qpkbp/1p1p1pp1/2p1N3/2P1P3/P1N1B2P/1PP2PP1/2KR3R b - - 0 14".to_string());
        assert!(engine.minimax_naive(&NaiveEval, 1, true) > 0.0);
        assert!(engine.minimax_naive(&NaiveEval, 2, true) > 0.0);
    }
}
