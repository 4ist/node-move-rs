mod node;
use crate::node::Node;
use crate::node::NodeType;

mod tile;
use crate::tile::Tile;
use crate::tile::TileType;

mod grid;
use crate::grid::Grid;

mod display;
use crate::display::display;

fn main() {
    println!("Hello, worlt!");
    display();
    println!("Goodbye");
}

