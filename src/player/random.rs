use rand::{thread_rng, seq::IteratorRandom};

use crate::game::{GameState, Edge, EdgeState};
use super::Player;

/// A random player which chooses some completely random edge. Even if that
/// edge will make this player loose the game.
pub struct DumbRandom;

impl Player for DumbRandom {
    fn new(_: EdgeState) -> Self
    where
        Self: Sized
    {
        Self
    }

    fn next_move(&mut self, state: &GameState) -> Edge {
        random_available_move(state)
    }
}

/// A random player that always chooses a edge that won't lead to immediate
/// loss if such an edge is available.
pub struct Random(EdgeState);

impl Player for Random {
    fn new(color: EdgeState) -> Self
    where
        Self: Sized
    {
        Self(color)
    }

    fn next_move(&mut self, state: &GameState) -> Edge {
        // First try to find an edge that won't lead to loosing the game. If
        // that's not possible, just take a random other one.
        Edge::all_edges()
            .filter(|e| state.edge_state(*e).is_none())
            .filter(|e| !state.would_create_triangle(*e, self.0))
            .choose(&mut thread_rng())
            .unwrap_or(random_available_move(state))
    }
}

/// Returns a random edge that is still `None`. Panics if all edges in `state`
/// are already colored.
fn random_available_move(state: &GameState) -> Edge {
    Edge::all_edges()
        .filter(|e| state.edge_state(*e).is_none())
        .choose(&mut thread_rng())
        .unwrap()
}
