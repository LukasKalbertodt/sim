use std::{
    sync::mpsc::Receiver,
};
use quicksilver::{
    Result,
    geom::{Circle, Line},
    graphics::{Background::Col, Color},
    lifecycle::{State, Window},
};


use crate::{
    GameState, EdgeId, EdgeState,
};

pub(crate) struct SimDisplay {
    pub(crate) last_state: GameState,
    pub(crate) new_states: Receiver<GameState>,
}

impl State for SimDisplay {
    fn new() -> Result<SimDisplay> {
        unimplemented!()
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        while let Ok(state) = self.new_states.try_recv() {
            self.last_state = state;
        }

        window.clear(Color::BLACK)?;

        let point_color = Col(Color::WHITE);
        let points = [
            (350, 250),
            (650, 250),
            (800, 500),
            (650, 750),
            (350, 750),
            (200, 500),
        ];

        // Draw all edges
        for e in (0..15).map(EdgeId::new) {
            let (color, width) = match self.last_state.edge_state(e) {
                EdgeState::None => (Col(Color { r: 0.6, g: 0.6, b: 0.6, a: 1.0 }), 1.5),
                EdgeState::Red => (Col(Color { r: 0.9, g: 0.29, b: 0.23, a: 1.0 }), 4.0),
                EdgeState::Blue => (Col(Color { r: 0.3, g: 0.49, b: 1.0, a: 1.0 }), 6.0),
            };

            let (va, vb) = e.endpoints();
            let pa = points[va.0 as usize];
            let pb = points[vb.0 as usize];
            window.draw(&Line::new(pa, pb).with_thickness(width), color);
        }

        // Draw all points
        for &p in &points {
            window.draw(&Circle::new(p, 10), point_color);
        }

        Ok(())
    }
}
