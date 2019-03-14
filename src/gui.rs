//! A GUI for playing the SIM game.

use quicksilver::{
    Error,
    geom::{Circle, Line, Vector},
    graphics::{Background, Color},
    input::{ButtonState, MouseButton, MouseCursor, Key},
    lifecycle::{Event, State, Window},
};


use crate::{
    player::Player,
    game::{GameState, Edge, EdgeState},
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


/// A `quicksilver` state which controls the full game (polling players for
/// moves or getting user input).
pub(crate) struct GuiGame {
    state: GameState,
    player_red: Option<Box<dyn Player>>,
    player_blue: Option<Box<dyn Player>>,

    reds_turn: bool,
    game_end: bool,

    /// The edge under the mouse cursor (if any).
    hovered_edge: Option<Edge>,
}

impl GuiGame {
    pub(crate) fn new(
        player_red: Option<Box<dyn Player>>,
        player_blue: Option<Box<dyn Player>>,
    ) -> Self {
        Self {
            state: GameState::new(),
            player_red,
            player_blue,

            reds_turn: true,
            game_end: false,

            hovered_edge: None,
        }
    }

    /// Checks if we are waiting for user input. This is the case when it's a
    /// `None` player's turn.
    fn waiting_for_input(&self) -> bool {
        if self.game_end {
            return false;
        }

        if self.reds_turn {
            self.player_red.is_none()
        } else {
            self.player_blue.is_none()
        }
    }

    /// Returns the color of the current player.
    fn active_color(&self) -> EdgeState {
        if self.reds_turn {
            EdgeState::Red
        } else {
            EdgeState::Blue
        }
    }

    /// Colors the given edge in the color of the active player.
    fn execute_move(&mut self, edge: Edge) {
        // Check if the game ends
        if self.state.would_create_triangle(edge, self.active_color()) {
            let winner = if self.reds_turn { "Blue" } else { "Red" };
            println!("Player {} won!", winner);
            self.game_end = true;
        }

        self.state.set_edge(edge, self.active_color());
        self.reds_turn = !self.reds_turn;
    }
}

impl State for GuiGame {
    fn new() -> Result<GuiGame, Error> {
        // I would say this is fairly bad API design from `quicksilver`. This
        // `new` method is required by the trait `State`, but it's not used if
        // one uses `run_with` instead of `run`. There is no sensible way to
        // implement this method.
        panic!(
            "Called `GuiGame::new`: this method must not be called \
                (use `run_with` to create this state)"
        );
    }

    // Is called in regular intervals
    fn update(&mut self, _: &mut Window) -> Result<(), Error> {
        if !self.game_end {
            // Get the active player
            let player = if self.reds_turn {
                self.player_red.as_mut().map(|b| &mut **b)
            } else {
                self.player_blue.as_mut().map(|b| &mut **b)
            };

            // If the player is a non-human player, get a move and execute it.
            if let Some(player) = player {
                let edge = player.next_move(&self.state);
                self.execute_move(edge);
            }
        }

        Ok(())
    }

    // Is called each frame
    fn draw(&mut self, window: &mut Window) -> Result<(), Error> {
        window.clear(BACKGROUND_COLOR)?;

        // Draw all edges
        for e in Edge::all_edges() {
            let (color, width) = match self.state.edge_state(e) {
                EdgeState::None => {
                    if self.hovered_edge == Some(e) && self.waiting_for_input() {
                        (SELECTED_COLOR, 5.0)
                    } else {
                        (COLOR_GREY, 1.5)
                    }
                }
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
                self.hovered_edge = Edge::all_edges()
                    .filter(|e| self.state.edge_state(*e).is_none())
                    .find(|e| distance_to_point(*e, *new_pos) < HOVER_DISTANCE);

                let cursor = if self.hovered_edge.is_some() {
                    MouseCursor::Hand
                } else {
                    MouseCursor::Default
                };
                window.set_cursor(cursor);
            }

            // If the mouse button is pressed, we are waiting for input and the
            // mouse hovers over a line, that line is selected.
            Event::MouseButton(MouseButton::Left, ButtonState::Released)
                if self.waiting_for_input() =>
            {
                if let Some(hovered_edge) = self.hovered_edge {
                    self.execute_move(hovered_edge);
                }
            }
            _ => {}
        }

        Ok(())
    }
}

/// Calculates the nearest distance of the point `p` to the line segment
/// defined by `e`.
pub(crate) fn distance_to_point(e: Edge, p: Vector) -> f32 {
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
