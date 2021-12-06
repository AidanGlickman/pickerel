use crate::state::state::{Move, State, StateEval};
use ordered_float::OrderedFloat;
use std::cmp;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

pub trait Engine<M: Move, S: State<M>> {
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

    fn minimax_naive_recurse(
        &self,
        evaluator: &dyn StateEval<M, S>,
        depth: usize,
        perspective: bool,
        state: S,
    ) -> f64
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
                    state,
                )));
            }

            if perspective {
                return values.iter().max().unwrap().into_inner();
            } else {
                return values.iter().min().unwrap().into_inner();
            }
        }
    }
    // ################################################################################
    //                                  Simplified ABDADA
    // ################################################################################
    const DEFER_DEPTH: usize = 2;
    const NUM_THREADS: usize = 4;

    fn abdada(
        &self,
        alpha: usize,
        beta: usize,
        depth: usize,
        evaluator: &dyn StateEval<M, S>,
        state: S,
    ) -> usize {
        let mut curr_searching = Arc::new(RwLock::new(HashSet::new()));
        let mut position_vals = Arc::new(RwLock::new(HashSet::new()));
        let thereads = (0..Self::NUM_THREADS).map(|_| {
            let searching = Arc::clone(&curr_searching);
            let vals = Arc::clone(&position_vals);
            let val = self.abdada_recurse(
                usize::MIN,
                usize::MAX,
                depth,
                true,
                evaluator,
                state,
                vals,
                searching,
            );
        });
        0
    }

    fn abdada_recurse(
        &self,
        alpha: usize,
        beta: usize,
        depth: usize,
        maximizing: bool,
        evaluator: &dyn StateEval<M, S>,
        state: S,
        position_values: &HashMap<M>,
        curr_searching: &HashSet<M>,
    ) -> usize {
        let legal_moves: Vec<M> = state.legal_moves();
        if depth == 0 || legal_moves.is_empty() {
            return evaluator.evaluate(&state);
        }
        let mut curr_searching: HashSet<M> = HashSet::new();
        let mut deferred_moves: Vec<M> = vec![];
        let mut x: usize = 0;
        let comp = if maximizing { cmp::max } else { cmp::min };
        for i in 0..legal_moves.len() {
            if i == 0 {
                x = comp(
                    x,
                    self.abdada_recurse(
                        beta,
                        alpha,
                        depth - 1,
                        !maximizing,
                        evaluator,
                        state.make_move(legal_moves[i]),
                        &curr_searching,
                    ),
                );
            } else {
                if self.defer_move(legal_moves[i], depth, &curr_searching) {
                    deferred_moves.push(legal_moves[i]);
                    continue;
                }

                self.starting_search(legal_moves[i], depth, &curr_searching);
                x = comp(
                    x,
                    self.abdada_recurse(
                        alpha,
                        beta,
                        depth - 1,
                        !maximizing,
                        evaluator,
                        state.make_move(legal_moves[i]),
                        &curr_searching,
                    ),
                );
                self.finished_search(legal_moves[i], depth, &curr_searching);
            }

            if maximizing {
                if x >= beta {
                    break;
                }
                alpha = cmp::max(alpha, x);
            } else {
                if x <= alpha {
                    break;
                }
                beta = cmp::min(beta, x);
            }
        }
        return x;
    }

    fn defer_move(&self, curr_move: M, depth: usize, curr_searching: &HashSet<M>) -> bool {
        if depth < Self::DEFER_DEPTH {
            false
        } else {
            curr_searching.contains(&curr_move)
        }
    }

    fn starting_search(&self, curr_move: M, depth: usize, curr_searching: &HashSet<M>) {
        if depth < Self::DEFER_DEPTH {
            return;
        }
        curr_searching.insert(curr_move);
    }

    fn finished_search(&self, curr_move: M, depth: usize, curr_searching: &HashSet<M>) {
        if depth < Self::DEFER_DEPTH {
            return;
        }
        curr_searching.remove(&curr_move);
    }
}
