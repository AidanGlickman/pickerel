use crate::state::state::{Move, State, StateEval};

pub trait Engine<M: Move, S: State<M>> {
    // fn new() -> Self;
    fn legal_moves(&self) -> Vec<M>;
    fn score(&self, evaluator: &dyn StateEval<M, S>) -> f64;
    fn make_move(&self, _: M);
    // fn state(&self) -> &'static S;
}
