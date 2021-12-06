pub trait Move: std::cmp::Eq + std::hash::Hash + Copy {
    fn is_valid(self) -> bool;
}

// TODO: serde impls
pub trait State<M: Move>: Copy {
    fn is_valid(&self) -> bool;
    fn format(&self) -> String;
    fn hash(&self) -> u64;
    fn make_move(&self, _: M) -> Self;
    fn legal_moves(&self) -> Vec<M>;
}

pub trait StateEval<M: Move, S: State<M>>: Sync {
    fn evaluate(&self, _: &S) -> f64;
}
