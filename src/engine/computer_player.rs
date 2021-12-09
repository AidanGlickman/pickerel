use crate::engine::engine::Engine;
struct ComputerPlayer {
    engine: Engine
    evaluator: &dyn StateEval<M, S>,
    depth:
}