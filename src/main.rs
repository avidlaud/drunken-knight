mod canvas;
mod action;

use std::fs;
use sha3::{Digest, Sha3_512};
use clap::Parser;

use crate::canvas::Grid;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // If true, knight will jump off canvas and appear on the opposite side
    #[clap(short, long)]
    unbounded: bool,

    // Height of the canvas
    #[clap(default_value_t = 9, short, long, validator = is_valid_size)]
    height: usize,

    // Width of the canvas
    #[clap(default_value_t = 17, short, long, validator = is_valid_size)]
    width: usize,

    // File to hash
    filename: String,
}

fn main() {
    let args = Args::parse();

    let mut canvas = canvas::Canvas::new(canvas::FlatGrid::new(args.height, args.width));

    let bytes = fs::read(args.filename).unwrap();

    let mut hasher = Sha3_512::new();
    hasher.update(bytes);
    let hash = hasher.finalize();

    let actions : Vec<action::Action> = hash.iter()
        .flat_map(|b| [b >> 4, b & 0x0F])
        .map(action::get_action)
        .collect();
    
    
    for act in actions.iter() {
        canvas.simulate(act, args.unbounded)
    } 
    println!("{}", canvas);
}

fn is_valid_size(s: &str) -> Result<(), String> {
    match s.parse::<usize>() {
        Ok(v) => if v % 2 == 1 { Ok(()) } else { Err(format!("Size needs to be odd! {} is not valid.", v))}
        Err(e) => Err(format!("Size needs to be valid positive integer! {} is not valid.", e)),
    }
}