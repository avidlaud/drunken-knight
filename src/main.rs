mod canvas;
mod action;

use std::fs;
use sha3::{Digest, Sha3_512};

use crate::canvas::Grid;

fn main() {
    let mut canvas = canvas::Canvas::new(canvas::FlatGrid::new(7, 7));

    let bytes = fs::read("./key.gpg").unwrap();

    let mut hasher = Sha3_512::new();

    hasher.update(bytes);

    let hash = hasher.finalize();

    let actions : Vec<action::Action> = hash.iter()
        .flat_map(|b| [b >> 4, b & 0x0F])
        .map(action::get_action)
        .collect();
    
    for act in actions.iter() {
        canvas.simulate(act)
    } 
    println!("{}", canvas);
}
