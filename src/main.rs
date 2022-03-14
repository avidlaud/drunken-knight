mod canvas;

use crate::canvas::Grid;

fn main() {
    let canvas = canvas::Canvas::new(canvas::FlatGrid::new(7, 5));
    // dbg!(&canvas);
    println!("{}", canvas);
}
