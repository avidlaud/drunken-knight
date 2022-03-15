mod canvas;
mod action;

use crate::canvas::Grid;

fn main() {
    let mut canvas = canvas::Canvas::new(canvas::FlatGrid::new(17, 9));
    dbg!(&canvas);
    canvas.simulate(action::Action::Increment(action::Jump::Still));
    println!("{}", canvas);
}
