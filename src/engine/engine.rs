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
        let legal_moves: Vec<M> = self.legal_moves();
        if depth == 0 || legal_moves.is_empty() {
            evaluator.evaluate(&self.state())
        } else {
            let mut values = vec![];

            for movement in legal_moves {
                let state: Self = self.make_move(movement);
                values.push(OrderedFloat(state.minimax_naive(
                    evaluator,
                    depth - 1,
                    !perspective,
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
