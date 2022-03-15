mod canvas;
mod action;

use std::fs;

use crate::canvas::Grid;

fn main() {
    let mut canvas = canvas::Canvas::new(canvas::FlatGrid::new(17, 9));
    println!("{}", canvas);

    let bytes = fs::read("./key.gpg").unwrap();

    let actions : Vec<action::Action> = bytes.iter()
        .flat_map(|b| [b >> 4, b & 0x0F])
        .map(action::get_action)
        .collect();
    
    for act in actions.iter() {
        canvas.simulate(act)
    } 
    println!("{}", canvas);
}
