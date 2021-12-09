use std::io;

pub mod engine;
pub mod evaluators;
pub mod state;

use crate::engine::engine::Engine;
use crate::evaluators::null::NullEval;
use crate::evaluators::naive::NaiveEval;
use crate::state::state::{State, StateEval};

fn main() {

    let mut engine = engine::chess_engine::ChessEngine::new();
    let mut stdin = io::stdin(); // We get `Stdin` here.
    loop {
        let mut fen_buffer = String::new();
        stdin.read_line(&mut fen_buffer);
        engine.update_fen(fen_buffer);
        println!("Material advantage: {}", NaiveEval.evaluate(&engine.board));
        println!("Minimax naive (depth = 3): {}", engine.minimax_naive(&NaiveEval, 3, true));
        println!("Abdada, with alpha-beta pruning (depth = 6) {}", engine.abdada(&NaiveEval, 6));
    }
}
