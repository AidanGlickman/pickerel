pub mod engine;
pub mod evaluators;
pub mod state;

use crate::engine::engine::Engine;
use crate::evaluators::null::NullEval;
use crate::evaluators::naive::NaiveEval;
use crate::state::state::State;

fn main() {
    let engine = engine::chess_engine::ChessEngine::from_str("Q5nr/p2qpkbp/1p1p1pp1/2p1N3/2P1P3/P1N1B2P/1PP2PP1/2KR3R b - - 0 14".to_string());
    println!("{}", engine.board.hash());
    println!("{}", engine.minimax_naive(&NaiveEval, 2, true));
    // println!("{}", engine.minimax_naive(&NaiveEval, 5, true));
    println!("{}", engine.abdada(&NaiveEval, 15));
}
