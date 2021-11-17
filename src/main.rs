pub mod engine;
pub mod evaluators;
pub mod state;

use crate::engine::engine::Engine;
use crate::evaluators::null::NullEval;
use crate::state::state::State;

fn main() {
    let engine = engine::chess_engine::ChessEngine::new();
    println!("{}", engine.board.hash());
    println!("{}", engine.minimax_naive(&NullEval, 2, true));
}
