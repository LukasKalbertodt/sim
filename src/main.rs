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

mod game;
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
}
