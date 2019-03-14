//! Everything related to visualising the Sim game state.

use std::{
    sync::mpsc::Receiver,
};
use quicksilver::{
    Error,
    geom::{Circle, Line},
    graphics::{Background, Color},
    lifecycle::{State, Window},
};


use crate::{
    GameState, EdgeId, EdgeState,
};

/// A `quicksilver` state that displays a Sim game state.
pub(crate) struct SimDisplay {
    pub(crate) last_state: GameState,
    pub(crate) new_states: Receiver<GameState>,
}


const COLOR_GREY: Color = Color { r: 0.6, g: 0.6,  b: 0.6,  a: 1.0 };
const COLOR_RED: Color = Color {  r: 0.9, g: 0.29, b: 0.23, a: 1.0 };
const COLOR_BLUE: Color = Color { r: 0.3, g: 0.49, b: 1.0,  a: 1.0 };
const POINT_COLOR: Color = Color::WHITE;
const BACKGROUND_COLOR: Color = Color::BLACK;


impl State for SimDisplay {
    fn new() -> Result<SimDisplay, Error> {
        panic!(
            "Called `SimDisplay::new`: this method must not be called \
                (use `run_with` to create this state)"
        );
    }

    fn draw(&mut self, window: &mut Window) -> Result<(), Error> {
        while let Ok(state) = self.new_states.try_recv() {
            self.last_state = state;
        }

        window.clear(BACKGROUND_COLOR)?;

        // We have a flat-top hexagon:
        //
        //       _____
        //      /     \
        //     /       \
        //     \       /
        //      \_____/
        //
        // We want a margin of 50 on each side. Since our canvas has the size
        // 1000, that means our outer radius R (the bigger one, from the center
        // to the points) is (1000 - 2 * 50) / 2 = 450. The inner radius r
        // (perpendicular to the edges of the hexagon) is calculates as r =
        // sqrt(3)/2 * R ≈ 390.
        let points = [
            (275, 110),
            (725, 110),
            (950, 500),
            (725, 890),
            (275, 890),
            ( 50, 500),
        ];

        // Draw all edges
        for e in (0..15).map(EdgeId::new) {
            let (color, width) = match self.last_state.edge_state(e) {
                EdgeState::None => (COLOR_GREY, 1.5),
                EdgeState::Red => (COLOR_RED, 4.0),
                EdgeState::Blue => (COLOR_BLUE, 6.0),
            };

            let (va, vb) = e.endpoints();
            let pa = points[va.0 as usize];
            let pb = points[vb.0 as usize];
            window.draw(
                &Line::new(pa, pb).with_thickness(width),
                Background::Col(color),
            );
        }

        // Draw all points
        for &p in &points {
            window.draw(&Circle::new(p, 10), Background::Col(POINT_COLOR));
        }

        Ok(())
    }
}
