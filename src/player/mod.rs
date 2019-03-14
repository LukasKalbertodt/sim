use crate::{GameState, EdgeId, EdgeState};

pub(crate) mod human;


pub(crate) trait Player {
    fn new(color: EdgeState) -> Self
    where
        Self: Sized;
    fn get_move(&mut self, state: &GameState) -> EdgeId;
}
