use super::Player;
use crate::game::{Edge, EdgeState, GameState};
use crate::player::Random;
use std::mem;

pub struct MiniMax(EdgeState);

impl MiniMax {
    // Wanted to do recursive minimax function at first, but was worried about
    // the overhead, so this ugly thing came out in the end
    fn mini_max_move(&mut self, state: &mut GameState) -> Edge {
        // gather our possible moves
        let mut encoded_moves = MiniMax::get_encoded_moves(state);

        // gather all edges so we dont have to keep creating them on the fly
        let edges: [Edge; 15] = [
            Edge::new(0),
            Edge::new(1),
            Edge::new(2),
            Edge::new(3),
            Edge::new(4),
            Edge::new(5),
            Edge::new(6),
            Edge::new(7),
            Edge::new(8),
            Edge::new(9),
            Edge::new(10),
            Edge::new(11),
            Edge::new(12),
            Edge::new(13),
            Edge::new(14),
        ];

        // For convenient access to player handles
        let me = self.0;
        let other = if me == EdgeState::Red {
            EdgeState::Blue
        } else {
            EdgeState::Red
        };

        // Using these to conveniently switch between acting and waiting
        let mut acting = me;
        let mut waiting = other;

        // Remember at which depth we started off, so when ascending upwards in the tree
        // we know where to stop
        let pre_depth = 15 - encoded_moves.count_ones() as usize;

        // First moves are all equal
        if pre_depth == 0 {
            let startmove = Random::new(me).next_move(state);
            if me == EdgeState::Blue {
                println!(
                    "Blue randomly choses starting move {}, expecting to lose",
                    startmove.id()
                );
            } else {
                println!(
                    "Red randomly choses starting move {}, expecting to lose",
                    startmove.id()
                );
            }
            return startmove;
        } else if pre_depth == 1 {
            // Second moves always win (?)
            let secondmove = Random::new(me).next_move(state);
            if me == EdgeState::Blue {
                println!(
                    "Blue thinks: second move always wins, chooses randomly: {}",
                    secondmove.id()
                );
            } else {
                println!(
                    "Red thinks: second move always wins, chooses randomly: {}",
                    secondmove.id()
                );
            }
            return secondmove;
        } else if pre_depth == 2 {
            // If second moves always win, we can choose random here and hope for a mistake
            // TODO this can probably improved so we dont do the worst move right away
            let thirdmove = Random::new(me).next_move(state);
            if me == EdgeState::Blue {
                println!(
                    "Blue knows no winning move, choses randomly: {} (at depth {})",
                    thirdmove.id(),
                    pre_depth
                );
            } else {
                println!(
                    "Red knows no winning move, choses randomly: {} (at depth {})",
                    thirdmove.id(),
                    pre_depth
                );
            }
            return thirdmove;
        }

        // Using this to track our progression through game rounds/tree depth
        let mut depth = pre_depth;

        // Using an array to represent the move sequence (this is basically our tree
        // thats being expanded)
        let mut move_sequence = [0; 15];

        // Using this to clearly represent the move thats currently being considered
        let mut current_move;

        // Using this to signal that we have just ascended in the tree (not descended)
        // and thus need to look for the next branch - or ascend further
        let mut ascend = false;

        // Using this to signal that we want to keep looking for more moves on the current level
        let mut samelevel = false;

        // This reflects the nature of the minimax problem, switching rounds means the players
        // change and aim to achieve a minimization/maximization (from the perspective of "me"
        // player)
        // In this case, as theres only 2 possible game outcomes (win, lose) we can reduce it
        // to bools.
        // "True": downwards a branch, "Me"-Player can force a win
        // "False": downwards a branch, "Other"-Player can force a win
        // "Me"-Player estimations will be initialized to "false" and then choose the maximum
        // of next lowest layer results (downgoing branches).
        // "Other"-Player estimations will be initialized to "true" and then choose the maximum
        // of next lowest layer results (downgoing branches).
        // Can replace minimization by "and"-ing the next lowest layer minimax result into the
        // current layer
        // Can replace maximization by "or"-ing the next lowest layer minimax result into the
        // current layer.
        // We can skip branches (in an alpha/beta-pruning sort of way) on estimation changes
        // from true -> false (definite minimum) and from false -> true (definite maximum)
        let mut minimax = [true; 15];

        // Track amount of expanded positions
        // let mut counter: u64 = 0;

        // Now descend depth-first through the move sequence tree and let the leaf results
        // propagate upwards our minimax structure
        loop {
            if depth > 14 {
                panic!("How could this happen to me");
            }

            // If we just ascended back to our starting depth, we might be done already
            if depth == pre_depth && ascend {
                // but only if we are sure we found a winning move
                if minimax[depth] {
                    if me == EdgeState::Blue {
                        println!(
                            "Blue knows the winning move: {} (at depth {})",
                            move_sequence[depth],
                            pre_depth
                        );
                    } else {
                        println!(
                            "Red knows the winning move: {} (at depth {})",
                            move_sequence[depth],
                            pre_depth
                        );
                    }
                    // println!("Expanded positions {}", counter);
                    return edges[move_sequence[depth] as usize];
                }
            }

            // Move we are currently considering is saved in its respective slot in the
            // move sequence
            current_move = move_sequence[depth];

            // If we just ascended from a deeper tree layer, we reset the move counter on
            // the next deeper tree level, undo the last move we made on this depth and
            // increment the move counter for this depth
            if ascend {
                move_sequence[depth + 1] = 0;
                encoded_moves = encoded_moves | (1 << current_move);
                state.set_edge(edges[current_move as usize], EdgeState::None);
                current_move += 1;
                ascend = false;
            } else if samelevel {
                // If at same level as before, choose next move
                current_move += 1;
                samelevel = false;
            } else {
                // If we descended, we reinit result tracking for this layer
                // counter += 1;
                move_sequence[depth] = 0;
                minimax[depth] = acting != me;
            }

            // See if theres any feasible move left
            while current_move != 15 && (encoded_moves & (1 << current_move)) == 0 {
                current_move += 1;
            }

            // If we already checked all moves for the current depth,
            // or we already found a move that wins us the game
            // then we ascend one layer up in the tree
            if current_move == 15
                || (acting == me && minimax[depth])
                || (acting == other && !minimax[depth])
            {
                ascend = true;
                // If we are out of moves at our starting depth, we didnt find a winning move
                if depth == pre_depth {
                    let randmove = Random::new(me).next_move(state);
                    if me == EdgeState::Blue {
                        println!(
                            "Blue knows no winning move, choses randomly: {} (at depth {})",
                            randmove.id(),
                            pre_depth
                        );
                    } else {
                        println!(
                            "Red knows no winning move, choses randomly: {} (at depth {})",
                            randmove.id(),
                            pre_depth
                        );
                    }
                    // println!("Expanded positions {}", counter);
                    return Random::new(me).next_move(state);
                }
            }

            // If current move would lose acting player the game (tree leaf)
            // he gotta keep looking for potentially better moves
            if !ascend && state.would_create_triangle(edges[current_move as usize], acting) {
                move_sequence[depth] = current_move;
                samelevel = true;
                continue;
            }

            // Do we need to ascend or descend into the tree? change depth and moves accordingly
            if ascend {
                move_sequence[depth] = 0;
                encoded_moves = encoded_moves | (1 << current_move);
                // the result from this layer is used for the next higher layer in a minimax way
                if acting == me {
                    minimax[depth - 1] = minimax[depth - 1] && minimax[depth];
                } else {
                    minimax[depth - 1] = minimax[depth - 1] || minimax[depth];
                }
                depth -= 1;
            } else {
                // Apply the move and go one step deeper
                move_sequence[depth] = current_move;
                encoded_moves = encoded_moves & !(1 << current_move);
                state.set_edge(edges[current_move as usize], acting);
                depth += 1;
            }

            mem::swap(&mut acting, &mut waiting);
        }
    }

    fn get_encoded_moves(state: &GameState) -> u16 {
        let mut encoded: u16 = 0;
        for index in 0..15 {
            if state.edge_state(Edge::new(index)) == EdgeState::None {
                encoded = encoded | (1 << index);
            }
        }
        encoded
    }
}

impl Player for MiniMax {
    fn new(color: EdgeState) -> Self
    where
        Self: Sized,
    {
        Self(color)
    }

    fn next_move(&mut self, state: &GameState) -> Edge {
        let mut state_copy = state.clone();
        self.mini_max_move(&mut state_copy)
    }
}
