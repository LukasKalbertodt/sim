use quicksilver::{
    geom::Vector,
    lifecycle::{Settings, run_with},
};
use structopt::StructOpt;

use crate::{
    game::EdgeState,
    gui::GuiGame,
    player::{Player, DumbRandom, Random},
};

mod game;
mod gui;
mod player;


fn main() {
    let opt = Opt::from_args();

    let player_red = get_player(&opt.player_red, EdgeState::Red);
    let player_blue = get_player(&opt.player_blue, EdgeState::Blue);

    run_with("Sim", Vector::new(1000, 1000), Settings::default(), || {
        Ok(GuiGame::new(player_red, player_blue))
    });
}

fn get_player(label: &str, color: EdgeState) -> Option<Box<dyn Player>> {
    match label {
        "human" => None,
        "random" => Some(Box::new(Random::new(color))),
        "dumb_random" => Some(Box::new(DumbRandom::new(color))),
        _ => panic!("unknown player label {}", label),
    }
}


#[derive(Debug, StructOpt)]
#[structopt(name = "sim", about = "Implementation of the Sim pencil game")]
struct Opt {
    /// The player with color red (the starting player).
    #[structopt(default_value = "human")]
    player_red: String,

    /// The player with color blue
    #[structopt(default_value = "random")]
    player_blue: String,
}
