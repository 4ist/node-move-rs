mod node;
use crate::node::Node;
use crate::node::NodeType;

mod tile;
use crate::tile::Tile;
use crate::tile::TileType;

mod cursor;
use crate::cursor::Cursor;

mod grid;
use crate::grid::Grid;
use crate::grid::Direction;

mod display;
use crate::display::display;

fn main() {
    println!("Hello, worlt!");
    display();
    println!("Goodbye");
}

