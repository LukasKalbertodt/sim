use crate::game::{GameState, Edge, EdgeState};

mod random;
mod minimax;

pub use random::{Random, DumbRandom};
pub use minimax::{MiniMax};

/// The interface for all non-human players.
pub(crate) trait Player {
    /// Create a new instance of the player.
    fn new(color: EdgeState) -> Self
    where
        Self: Sized;

    /// Return a new move (which edge to be colored).
    ///
    /// `state` is guaranteed to still have uncolored edges left.
    fn next_move(&mut self, state: &GameState) -> Edge;
}
