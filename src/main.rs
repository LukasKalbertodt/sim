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
    let player_red = opt.player_red.computer_player();
    let player_blue = opt.player_blue.computer_player();

    run_with("Sim", Vector::new(1000, 1000), Settings::default(), || {
        Ok(GuiGame::new(player_red, player_blue))
    });
}

/// A player choosen via command line.
enum PlayerInput {
    Human,
    Other(Box<dyn Player>),
}

impl PlayerInput {
    /// Returns the player or `None` if it's a human player.
    fn computer_player(self) -> Option<Box<dyn Player>> {
        match self {
            PlayerInput::Human => None,
            PlayerInput::Other(p) => Some(p),
        }
    }

    fn from_str(input: &str, color: EdgeState) -> Result<Self, String> {
        match input {
            "human" => Ok(PlayerInput::Human),
            "random" => Ok(PlayerInput::Other(Box::new(Random::new(color)))),
            "dumb_random" => Ok(PlayerInput::Other(Box::new(DumbRandom::new(color)))),
            _ => Err(format!(
                "invalid player '{}' (valid options: 'human', 'random', 'dumb_random')",
                input,
            )),
        }
    }
}

fn parse_player_red(input: &str) -> Result<PlayerInput, String> {
    PlayerInput::from_str(input, EdgeState::Red)
}

fn parse_player_blue(input: &str) -> Result<PlayerInput, String> {
    PlayerInput::from_str(input, EdgeState::Blue)
}

#[derive(StructOpt)]
#[structopt(
    name = "sim",
    about = "Implementation of the Sim pencil game",
    usage = "sim [FLAGS] <player_red> <player_blue>",
)]
struct Opt {
    /// The player with color red (the starting player). Possible values:
    /// 'human', 'random', 'random_dumb'.
    #[structopt(default_value = "human", parse(try_from_str = "parse_player_red"))]
    player_red: PlayerInput,

    /// The player with color blue.
    #[structopt(default_value = "random", parse(try_from_str = "parse_player_blue"))]
    player_blue: PlayerInput,
}
