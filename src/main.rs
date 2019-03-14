use quicksilver::{
    geom::Vector,
    lifecycle::{Settings, run_with},
};

use crate::gui::GuiGame;

mod game;
mod gui;
mod player;


fn main() {
    run_with("Sim", Vector::new(1000, 1000), Settings::default(), || {
        Ok(GuiGame::new(None, None))
    });
}
