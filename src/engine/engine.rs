use crate::state::state::{Move, State, StateEval};
use ordered_float::OrderedFloat;
use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use std::thread;

const DEFER_DEPTH: usize = 2;
const NUM_THREADS: usize = 4;

pub trait Engine<M: Move + std::marker::Sync + std::marker::Send, S: State<M> + std::marker::Send> {
    fn state(&self) -> S;
    fn legal_moves(&self) -> Vec<M>;
    fn score(&self, evaluator: &dyn StateEval<M, S>) -> f64;
    // fn make_move(&self, _: M) -> Self;

    fn add_to_cache(&mut self, state: S, score_depth: (f64, usize));
    fn get_from_cache(&self, state: &S) -> Option<(f64, usize)>;

    fn minimax_naive(&self, evaluator: &dyn StateEval<M, S>, depth: usize, perspective: bool) -> f64
    where
        Self: Sized,
    {
        minimax_naive_recurse(evaluator, depth, perspective, self.state())
    }

    // ################################################################################
    //                                  Simplified ABDADA
    // ################################################################################
    fn abdada(&self, depth: usize, evaluator: &dyn StateEval<M, S>, state: S) -> f64 {
        let curr_searching = Arc::new(RwLock::new(HashSet::new()));
        let threads: Vec<_> = (0..NUM_THREADS)
            .map(|_| {
                let searching = Arc::clone(&curr_searching);
                thread::spawn(move || {
                    abdada_recurse(
                        &self,
                        f64::MIN,
                        f64::MAX,
                        depth,
                        true,
                        evaluator,
                        &state,
                        &searching,
                    )
                })
            })
            .collect();
        for t in threads {
            t.join().expect("Thread panicked");
        }
        self.get_from_cache(&state).unwrap().0
    }
}

fn abdada_recurse<
    M: Move + std::marker::Sync + std::marker::Send,
    S: State<M> + std::marker::Send,
>(
    engine: &dyn Engine<M, S>,
    mut alpha: f64,
    mut beta: f64,
    depth: usize,
    maximizing: bool,
    evaluator: &dyn StateEval<M, S>,
    state: &S,
    curr_searching: &Arc<RwLock<HashSet<M>>>,
) -> f64 {
    match engine.get_from_cache(&state) {
        Some(score_depth) => {
            if score_depth.1 >= depth {
                return score_depth.0;
            }
        }
        None => (),
    }
    let mut x: f64 = 0.0; // always overwritten
    let legal_moves: Vec<M> = state.legal_moves();
    if depth == 0 || legal_moves.is_empty() {
        return evaluator.evaluate(&state);
    }
    let mut deferred_moves: Vec<M> = vec![];
    let mut values = vec![];
    for i in 0..legal_moves.len() {
        if i == 0 {
            values.push(OrderedFloat(abdada_recurse(
                engine,
                beta,
                alpha,
                depth - 1,
                !maximizing,
                evaluator,
                &state.make_move(legal_moves[i]),
                &curr_searching,
            )));
        } else {
            if defer_move(legal_moves[i], depth, &curr_searching) {
                deferred_moves.push(legal_moves[i]);
                continue;
            }

            starting_search(legal_moves[i], depth, &curr_searching);
            values.push(OrderedFloat(abdada_recurse(
                engine,
                beta,
                alpha,
                depth - 1,
                !maximizing,
                evaluator,
                &state.make_move(legal_moves[i]),
                &curr_searching,
            )));
            finished_search(legal_moves[i], depth, &curr_searching);
        }

        if maximizing {
            x = values.iter().max().unwrap().into_inner();
            if x >= beta {
                break;
            }
            alpha = alpha.max(x);
        } else {
            x = values.iter().min().unwrap().into_inner();
            if x <= alpha {
                break;
            }
            beta = beta.min(x);
        }
    }
    engine.add_to_cache(*state, (x, depth));
    x
}

fn defer_move<M: Move + std::marker::Sync + std::marker::Send>(
    curr_move: M,
    depth: usize,
    curr_searching: &Arc<RwLock<HashSet<M>>>,
) -> bool {
    if depth < DEFER_DEPTH {
        false
    } else {
        let set = curr_searching.read().expect("RwLock poisoned");
        set.contains(&curr_move)
    }
}

fn starting_search<M: Move + std::marker::Sync + std::marker::Send>(
    curr_move: M,
    depth: usize,
    curr_searching: &Arc<RwLock<HashSet<M>>>,
) {
    if depth < DEFER_DEPTH {
        return;
    }
    let mut set = curr_searching.write().expect("RwLock poisoned");
    set.insert(curr_move);
}

fn finished_search<M: Move + std::marker::Sync + std::marker::Send>(
    curr_move: M,
    depth: usize,
    curr_searching: &Arc<RwLock<HashSet<M>>>,
) {
    if depth < DEFER_DEPTH {
        return;
    }
    let mut set = curr_searching.write().expect("RwLock poisoned");
    set.insert(curr_move);
}

fn minimax_naive_recurse<
    M: Move + std::marker::Sync + std::marker::Send,
    S: State<M> + std::marker::Send,
>(
    evaluator: &dyn StateEval<M, S>,
    depth: usize,
    perspective: bool,
    state: S,
) -> f64 {
    let legal_moves: Vec<M> = state.legal_moves();
    if depth == 0 || legal_moves.is_empty() {
        evaluator.evaluate(&state)
    } else {
        let mut values = vec![];

        for movement in legal_moves {
            let state: S = state.make_move(movement);
            values.push(OrderedFloat(minimax_naive_recurse(
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
