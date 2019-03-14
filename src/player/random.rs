use rand::{thread_rng, seq::IteratorRandom};

use crate::game::{GameState, EdgeId, EdgeState};
use super::Player;


pub struct DumbRandom;

impl Player for DumbRandom {
    fn new(_: EdgeState) -> Self
    where
        Self: Sized
    {
        Self
    }

    fn get_move(&mut self, state: &GameState) -> EdgeId {
        random_available_move(state)
    }
}

pub struct Random(EdgeState);

impl Player for Random {
    fn new(color: EdgeState) -> Self
    where
        Self: Sized
    {
        Self(color)
    }

    fn get_move(&mut self, state: &GameState) -> EdgeId {
        // First try to find an edge that won't lead to loosing the game. If
        // that's not possible, just take a random other one.
        EdgeId::all_edges()
            .filter(|e| state.edge_state(*e).is_none())
            .filter(|e| !state.would_create_triangle(*e, self.0))
            .choose(&mut thread_rng())
            .unwrap_or(random_available_move(state))
    }
}

/// Returns a random edge that is still `None`. Panics if all edges in `state`
/// are already colored.
fn random_available_move(state: &GameState) -> EdgeId {
    EdgeId::all_edges()
        .filter(|e| state.edge_state(*e).is_none())
        .choose(&mut thread_rng())
        .unwrap()
}
