use std::{
    io::{stdout, stdin, Write, BufRead},
};

use crate::game::{GameState, EdgeId, VertexId, EdgeState};
use super::Player;


pub(crate) struct Human(EdgeState);

impl Player for Human {
    fn new(color: EdgeState) -> Self
    where
        Self: Sized
    {
        Self(color)
    }

    fn get_move(&mut self, state: &GameState) -> EdgeId {
        print!("Player {:?}, please give me ze edge endpoints: ", self.0);
        stdout().flush().unwrap();

        let mut s = String::new();
        loop {
            s.clear();
            stdin().lock().read_line(&mut s).unwrap();
            let v = s.trim().split_whitespace().collect::<Vec<_>>();

            if v.len() != 2 {
                println!("Errors hilfe nicht 2");
                continue;
            }

            match v[0].parse::<u8>().and_then(|a| v[1].parse::<u8>().map(|b| (a, b))) {
                Ok((a, b)) if a < 6 && b < 6 && a != b => {
                    let edge = EdgeId::between(VertexId::new(a), VertexId::new(b));
                    if !state.edge_state(edge).is_none() {
                        println!("Stille");
                        continue;
                    }
                    return edge;
                }
                Ok((a, b)) => {
                    println!("Nein Nein! {} oder {} ist zu hoch oder die sind gleich!", a, b);
                }
                Err(e) => {
                    println!("Cannot parse ur input yo: {}", e);
                }
            }
        }
    }
}
