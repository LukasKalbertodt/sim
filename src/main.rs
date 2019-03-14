use std::{
    mem,
    thread,
    sync::mpsc::channel,
};

use quicksilver::{
    geom::Vector,
    lifecycle::{Settings, run_with},
};

use crate::{
    gfx::SimDisplay,
    player::{
        Player,
        human::Human,
    },
};

mod gfx;
mod player;



fn main() {
    let (sender, receiver) = channel();

    let thread_handle = thread::spawn(move || {
        run_with("Draw Geometry", Vector::new(1000, 1000), Settings::default(), || {
            Ok(SimDisplay {
                last_state: GameState::new(),
                new_states: receiver,
            })
        });
    });

    let mut active_player = (Human::new(EdgeState::Red), EdgeState::Red);
    let mut inactive_player = (Human::new(EdgeState::Blue), EdgeState::Blue);
    let mut state = GameState::new();

    while !state.finished() {
        let (player, color) = &mut active_player;
        let e = player.get_move(&state);

        state.set_edge(e, *color);
        sender.send(state.clone()).unwrap();

        mem::swap(&mut active_player, &mut inactive_player);
    }
}

#[derive(Clone)]
struct GameState {
    ///
    ///
    /// 15 14 ... 1 0
    encoded: u32,
}

impl GameState {
    fn new() -> Self {
        Self {
            encoded: 0,
        }
    }

    fn edge_state(&self, id: EdgeId) -> EdgeState {
        let bits = (self.encoded >> (id.0 * 2)) & 0b11;
        match bits {
            0 => EdgeState::None,
            1 => EdgeState::Red,
            2 => EdgeState::Blue,
            _ => unreachable!(),
        }
    }

    fn set_edge(&mut self, id: EdgeId, state: EdgeState) {
        let bits = state as u8 as u32;

        // Example (id is 3, state is 2):
        //
        // original encoded:        ee dd cc bb aa
        // mask:                 &  11 00 11 11 11
        // shifted_bits:         |  00 10 00 00 00
        //                     -------------------------
        // result:                  ee 10 cc bb aa
        let mask = !(0b11 << (id.0 * 2));
        let shifted_bits = bits << (id.0 * 2);
        self.encoded = (self.encoded & mask) | shifted_bits;
    }
/*
    fn would_create_triangle(&self, edge: EdgeId, state: EdgeState) -> bool {
        fn pair(a: u8, b: u8) -> (EdgeId, EdgeId) {
            (EdgeId::new(a), EdgeId::new(b))
        }

        const TRIANGLES: [[(EdgeId, EdgeId); 4]; 15] = [
            [pair(5, 1), pair(8, 4), pair(7, 3), pair(6, 2)],
            [pair(0, 5), pair(3, 10), pair(11, 4), pair(9, 2)],
            [pair(0, 6), pair(3, 12), pair(4, 13), pair(9, 1)],
            [pair(12, 2), pair(4, 14), pair(1, 10), pair(7, 0)],
            [pair(13, 2), pair(11, 1), pair(8, 0), pair(14, 3)],
            [pair(), pair(), pair(), pair()],
            [pair(), pair(), pair(), pair()],
            [pair(), pair(), pair(), pair()],
            [pair(), pair(), pair(), pair()],
            [pair(), pair(), pair(), pair()],
            [pair(), pair(), pair(), pair()],
            [pair(), pair(), pair(), pair()],
            [pair(), pair(), pair(), pair()],
            [pair(), pair(), pair(), pair()],
            [pair(), pair(), pair(), pair()],
        ];
    }
*/
    fn finished(&self) -> bool {
        false
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
enum EdgeState {
    None = 0,
    Red = 1,
    Blue = 2,
}


impl EdgeState {
    fn is_none(&self) -> bool {
        *self == EdgeState::None
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct EdgeId(u8);

impl EdgeId {
    fn new(v: u8) -> Self {
        assert!(v < 15);
        Self(v)
    }

    fn all_edges() -> impl Iterator<Item = Self> {
        (0..15).map(Self::new)
    }

    fn between(a: VertexId, b: VertexId) -> Self {
        match (a.0, b.0) {
            (0, 1) | (1, 0) => Self::new(0),
            (0, 2) | (2, 0) => Self::new(1),
            (0, 3) | (3, 0) => Self::new(2),
            (0, 4) | (4, 0) => Self::new(3),
            (0, 5) | (5, 0) => Self::new(4),
            (1, 2) | (2, 1) => Self::new(5),
            (1, 3) | (3, 1) => Self::new(6),
            (1, 4) | (4, 1) => Self::new(7),
            (1, 5) | (5, 1) => Self::new(8),
            (2, 3) | (3, 2) => Self::new(9),
            (2, 4) | (4, 2) => Self::new(10),
            (2, 5) | (5, 2) => Self::new(11),
            (3, 4) | (4, 3) => Self::new(12),
            (3, 5) | (5, 3) => Self::new(13),
            (4, 5) | (5, 4) => Self::new(14),
            _ => unreachable!(),
        }
    }

    fn endpoints(&self) -> (VertexId, VertexId) {
        match self.0 {
            0 => (VertexId::new(0), VertexId::new(1)),
            1 => (VertexId::new(0), VertexId::new(2)),
            2 => (VertexId::new(0), VertexId::new(3)),
            3 => (VertexId::new(0), VertexId::new(4)),
            4 => (VertexId::new(0), VertexId::new(5)),
            5 => (VertexId::new(1), VertexId::new(2)),
            6 => (VertexId::new(1), VertexId::new(3)),
            7 => (VertexId::new(1), VertexId::new(4)),
            8 => (VertexId::new(1), VertexId::new(5)),
            9 => (VertexId::new(2), VertexId::new(3)),
            10 => (VertexId::new(2), VertexId::new(4)),
            11 => (VertexId::new(2), VertexId::new(5)),
            12 => (VertexId::new(3), VertexId::new(4)),
            13 => (VertexId::new(3), VertexId::new(5)),
            14 => (VertexId::new(4), VertexId::new(5)),
            _ => unreachable!(),
        }
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct VertexId(u8);

impl VertexId {
    fn new(v: u8) -> Self {
        assert!(v < 6);
        Self(v)
    }
}
