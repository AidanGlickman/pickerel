use crate::state::state::{Move, State, StateEval};
use ordered_float::OrderedFloat;

pub trait Engine<M: Move, S: State<M>> {
    // fn new() -> Self;
    fn state(&self) -> S;
    fn legal_moves(&self) -> Vec<M>;
    fn score(&self, evaluator: &dyn StateEval<M, S>) -> f64;
    fn make_move(&self, _: M) -> Self;

    fn minimax_naive(&self, evaluator: &dyn StateEval<M, S>, depth: usize, perspective: bool) -> f64
    where
        Self: Sized,
    {
        self.minimax_naive_recurse(evaluator, depth, perspective, self.state())
    }

    fn minimax_naive_recurse(&self, evaluator: &dyn StateEval<M, S>, depth: usize, perspective: bool, state: S) -> f64
    where
        Self: Sized,
    {
        let legal_moves: Vec<M> = state.legal_moves();
        if depth == 0 || legal_moves.is_empty() {
            evaluator.evaluate(&state)
        } else {
            let mut values = vec![];

            for movement in legal_moves {
                let state: S = state.make_move(movement);
                values.push(OrderedFloat(self.minimax_naive_recurse(
                    evaluator,
                    depth - 1,
                    !perspective,
                    state
                )));
            }

            if perspective {
                return values.iter().max().unwrap().into_inner();
            } else {
                return values.iter().min().unwrap().into_inner();
            }
        }
    }
    // fn state(&self) -> &'static S;
}
