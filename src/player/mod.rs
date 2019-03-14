use crate::game::{GameState, EdgeId, EdgeState};


pub(crate) trait Player {
    fn new(color: EdgeState) -> Self
    where
        Self: Sized;
    fn get_move(&mut self, state: &GameState) -> EdgeId;
}
