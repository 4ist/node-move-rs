mod node;
use crate::node::Node;
mod tile;
use crate::tile::Tile;
use crate::tile::TileType;
mod grid;
use crate::grid::Grid;
mod display;
use crate::display::display;
// use crate::node::Node;

fn main() {
    println!("Hello, worlt!");

    display();
    println!("Goodbye");
}

