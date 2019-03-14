//! Everything related to visualising the Sim game state.

use std::{
    sync::mpsc::Receiver,
};
use quicksilver::{
    Error,
    geom::{Circle, Line, Vector},
    graphics::{Background, Color},
    input::{ButtonState, MouseCursor, Key},
    lifecycle::{Event, State, Window},
};


use crate::{
    game::{GameState, EdgeId, EdgeState},
};


const COLOR_GREY: Color = Color { r: 0.6, g: 0.6,  b: 0.6,  a: 1.0 };
const COLOR_RED: Color = Color {  r: 0.9, g: 0.29, b: 0.23, a: 1.0 };
const COLOR_BLUE: Color = Color { r: 0.3, g: 0.49, b: 1.0,  a: 1.0 };
const SELECTED_COLOR: Color = Color { r: 0.6, g: 0.8, b: 0.6, a: 1.0 };
const POINT_COLOR: Color = Color::WHITE;
const BACKGROUND_COLOR: Color = Color::BLACK;

/// If the mouse cursor is closer to a line than this distance, we say the
/// mouse hovers over the line.
const HOVER_DISTANCE: f32 = 12.0;

/// We have a flat-top hexagon:
///
///       _____
///      /     \
///     /       \
///     \       /
///      \_____/
///
/// We want a margin of 50 on each side. Since our canvas has the size
/// 1000, that means our outer radius R (the bigger one, from the center
/// to the points) is (1000 - 2 * 50) / 2 = 450. The inner radius r
/// (perpendicular to the edges of the hexagon) is calculates as r =
/// sqrt(3)/2 * R ≈ 390.
const CORNER_POSITIONS: [Vector; 6] = [
    Vector { x: 275.0, y: 110.0 },
    Vector { x: 725.0, y: 110.0 },
    Vector { x: 950.0, y: 500.0 },
    Vector { x: 725.0, y: 890.0 },
    Vector { x: 275.0, y: 890.0 },
    Vector { x:  50.0, y: 500.0 },
];


/// A `quicksilver` state that displays a Sim game state.
pub(crate) struct SimDisplay {
    last_state: GameState,
    new_states: Receiver<GameState>,
    selected_edge: Option<EdgeId>,
}

impl SimDisplay {
    pub(crate) fn new(new_states: Receiver<GameState>) -> Self {
        Self {
            last_state: GameState::new(),
            new_states,
            selected_edge: None,
        }
    }
}

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

        // Draw all edges
        for e in EdgeId::all_edges() {
            let (color, width) = match self.last_state.edge_state(e) {
                EdgeState::None if self.selected_edge == Some(e) => (SELECTED_COLOR, 5.0),
                EdgeState::None => (COLOR_GREY, 1.5),
                EdgeState::Red => (COLOR_RED, 4.0),
                EdgeState::Blue => (COLOR_BLUE, 6.0),
            };

            let (va, vb) = e.endpoints();
            let pa = CORNER_POSITIONS[va.id() as usize];
            let pb = CORNER_POSITIONS[vb.id() as usize];
            window.draw(
                &Line::new(pa, pb).with_thickness(width),
                Background::Col(color),
            );
        }

        // Draw all points
        for &p in &CORNER_POSITIONS {
            window.draw(&Circle::new(p, 10), Background::Col(POINT_COLOR));
        }

        Ok(())
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<(), Error> {
        match event {
            Event::Key(Key::Escape, ButtonState::Pressed) => {
                window.close();
            }
            Event::MouseMoved(new_pos) => {
                self.selected_edge = EdgeId::all_edges()
                    .filter(|e| self.last_state.edge_state(*e).is_none())
                    .find(|e| distance_to_point(*e, *new_pos) < HOVER_DISTANCE);

                let cursor = if self.selected_edge.is_some() {
                    MouseCursor::Hand
                } else {
                    MouseCursor::Default
                };
                window.set_cursor(cursor);
            }
            _ => {}
        }

        Ok(())
    }
}


pub(crate) fn distance_to_point(e: EdgeId, p: Vector) -> f32 {
    // Get the edges endpoints
    let (va, vb) = e.endpoints();
    let a = CORNER_POSITIONS[va.id() as usize];
    let b = CORNER_POSITIONS[vb.id() as usize];

    // We pretend that `a` is the origin by subtracting a.
    let a_to_b = b - a;
    let a_to_p = p - a;

    // The idea is to project P onto the line via the dot product. But by
    // clamping the result between 0 and 1 we can make sure that P is projected
    // on the line segment instead. The distance between that resulting point
    // and the original point is the correct distance.
    //
    //     B                     B
    //     |                     |
    //     |    P    project     |----P
    //     |   /      ====>      |   /
    //     |  /                  |  /
    //     | /                   | /
    //     A                     A
    //

    // First we calculate the dot product and divide it by the distance from a
    // to b. If that value is between 0 and 1 it means that the projected
    // points lies on the line segment.
    let scale = a_to_b.dot(a_to_p) / a_to_b.len2();

    // Then we clamp this scale between 0 and 1 to make sure it lies on the
    // line segment.
    let scale = match scale {
        v if v < 0.0 => 0.0,
        v if v > 1.0 => 1.0,
        v => v,
    };

    // Actually calculate the projected point
    let projected_on_line_segment = a_to_b * scale;

    // Distance from projected point to P is our result
    a_to_p.distance(projected_on_line_segment)
}
