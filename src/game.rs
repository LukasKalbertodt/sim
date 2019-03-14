//! Types and functions describing the core game.

/// Stores the state of a Sim game (the state of all 15 edges).
#[derive(Clone)]
pub struct GameState {
    /// We use a very space effient way to store the game state.
    ///
    /// We have 15 edges and each edge can be in one of three states. Thus we
    /// need only 2 bits for each edge. This means we can fit all data into a
    /// `u32`. The LSB store the state of the edge with ID 0. The two
    /// upppermost bits are always 0.
    ///
    /// Bit:     31   30   29   28   27   26   …    3    2    1    0
    ///        ┌────┬────┬────┬────┬────┬────┬───┬────┬────┬────┬────┐
    /// Edge:  │ __ │ __ │ 14 │ 14 │ 13 │ 13 │ … │  1 │  1 │  0 │  0 │
    ///        └────┴────┴────┴────┴────┴────┴───┴────┴────┴────┴────┘
    ///
    encoded: u32,
}

impl GameState {
    /// Returns a new game where all edges are `None`.
    pub fn new() -> Self {
        Self {
            encoded: 0,
        }
    }

    /// Returns the state of the given edge.
    pub fn edge_state(&self, id: EdgeId) -> EdgeState {
        // First we shift the bits to the right so that the relevant two bits
        // are the LSBs. Then we mask of the other stuff.
        match (self.encoded >> (id.id() * 2)) & 0b11 {
            0 => EdgeState::None,
            1 => EdgeState::Red,
            2 => EdgeState::Blue,
            _ => unreachable!(),
        }
    }

    pub fn set_edge(&mut self, id: EdgeId, state: EdgeState) {
        let bits = state as u8 as u32;

        // We need to set two bits without touching any other bits. We do this
        // by first setting the relevant bits to 0 and then set the value by
        // or-ing. Example (id is 3, state is 2 = 0b10):
        //
        // original encoded:        ee dd cc bb aa
        // mask:                 &  11 00 11 11 11
        // shifted_bits:         |  00 10 00 00 00
        //                     -------------------------
        // result:                  ee 10 cc bb aa
        let mask = !(0b11 << (id.id() * 2));
        let shifted_bits = bits << (id.id() * 2);
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
    pub fn finished(&self) -> bool {
        false
    }
}

/// Represents an edge. It can either be uncolored (`None`) or be colored by
/// one player.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EdgeState {
    None = 0,
    Red = 1,
    Blue = 2,
}

impl EdgeState {
    /// Returns `true` if this edge is still uncolored.
    pub fn is_none(&self) -> bool {
        *self == EdgeState::None
    }
}

/// ID of an edge (0 to 14 inclusive).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EdgeId(u8);

impl EdgeId {
    /// Creates a new ID from the given integer. `v` has to be less than 15!
    pub fn new(v: u8) -> Self {
        assert!(v < 15);
        Self(v)
    }

    /// Returns the inner ID.
    pub fn id(&self) -> u8 {
        self.0
    }

    /// Returns an iterator over all edges.
    pub fn all_edges() -> impl Iterator<Item = Self> {
        (0..15).map(Self::new)
    }

    /// Returns the edge between the two given vertices.
    pub fn between(a: VertexId, b: VertexId) -> Self {
        match (a.id(), b.id()) {
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

    /// Returns the IDs of the two endpoints of this edge.
    pub fn endpoints(&self) -> (VertexId, VertexId) {
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


/// ID of a vertex (0 to 5 inclusive).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VertexId(u8);

impl VertexId {
    /// Creates a new ID. `v` has to be less than 6!
    pub fn new(v: u8) -> Self {
        assert!(v < 6);
        Self(v)
    }

    /// Returns the inner ID.
    pub fn id(&self) -> u8 {
        self.0
    }
}
