use std::hash::Hash;

pub trait Move {
    fn is_valid(self) -> bool;
}

// TODO: serde impls
pub trait State<M: Move> {
    fn is_valid(&self) -> bool;
    fn format(&self) -> String;
    fn hash(&self) -> u64;
    fn make_move(&self, _: M);
    fn legal_moves(&self) -> Vec<M>;
}

pub trait StateEval<M: Move, S: State<M>> {
    fn evaluate(&self, _: &S) -> f64;
}
