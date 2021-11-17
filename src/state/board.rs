use chess::{Board, ChessMove, MoveGen};
// use serde::{Serialize, Deserialize};
use crate::state::state::{Move, State};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct ChessMoveWrapper {
    chess_move: ChessMove,
}

impl ChessMoveWrapper {
    fn new(movement: ChessMove) -> Self {
        ChessMoveWrapper {
            chess_move: movement,
        }
    }
}

impl Move for ChessMoveWrapper {
    fn is_valid(self) -> bool {
        true
    }
}

#[derive(Clone, Debug)]
pub struct ChessBoard {
    board: chess::Board,
}

impl ChessBoard {
    pub fn default() -> Self {
        let chess_board: Board = Board::default();
        ChessBoard { board: chess_board }
    }
}

impl State<ChessMoveWrapper> for ChessBoard {
    fn is_valid(&self) -> bool {
        self.board.is_sane()
    }

    fn format(&self) -> String {
        unimplemented!();
    }

    fn hash(&self) -> u64 {
        self.board.get_hash()
    }

    fn make_move(&self, movement: ChessMoveWrapper) -> ChessBoard {
        return ChessBoard {
            board: self.board.make_move_new(movement.chess_move),
        };
    }

    fn legal_moves(&self) -> Vec<ChessMoveWrapper> {
        MoveGen::new_legal(&(self.board))
            .map(ChessMoveWrapper::new)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::state::state::State;

    #[test]
    fn deterministic_hash() {
        let engine = crate::engine::chess_engine::ChessEngine::new();
        assert!(engine.board.hash() == engine.board.hash());
        let hash_1: u64 = engine.board.hash();

        let engine = crate::engine::chess_engine::ChessEngine::new();
        assert!(engine.board.hash() == engine.board.hash());
        let hash_2: u64 = engine.board.hash();

        assert!(hash_1 == hash_2);
    }
}
