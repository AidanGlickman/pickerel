use crate::state::board::ChessBoard;

pub fn check_end_pos(board: &ChessBoard) -> Option<f64>{
    match board.board.status() {
        chess::BoardStatus::Ongoing => None,
        chess::BoardStatus::Stalemate => Some(0.0),
        chess::BoardStatus::Checkmate => {
            match board.board.side_to_move() {
                chess::Color::White => Some(f64::MIN),
                chess::Color::Black => Some(f64::MAX)
            }
        }

    }
}