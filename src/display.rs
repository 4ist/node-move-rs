use nannou::prelude::*;

use crate::NodeType;
use crate::Grid;
use crate::Cursor;
use crate::Node;
use crate::Direction;
use crate::TileType;
use crate::Tile;

pub fn display() {
    nannou::app(model).update(update).run();
}


const WINDOW_SIZE_PX_X: u32 = 1500;
const WINDOW_SIZE_PX_Y: u32 = 900;
const TILE_SIZE_PX: u32 = 32;

const GRID_SIZE_X: i32 = (WINDOW_SIZE_PX_X / TILE_SIZE_PX) as i32;
const GRID_SIZE_Y: i32 = (WINDOW_SIZE_PX_Y / TILE_SIZE_PX) as i32;

struct Model {
    grid: Grid,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(WINDOW_SIZE_PX_X, WINDOW_SIZE_PX_Y)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let mut grid = Grid::new(GRID_SIZE_X, GRID_SIZE_Y); 
    grid.setup_add_tile_types();
    grid.setup_add_node_cursor();
    grid.setup_add_nodes_terrain();
    //grid.setup_add_gold_tile();
    Model { grid }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for _x in 0..model.grid.size_x{
        for _y in 0..model.grid.size_y {
            // Update state here if adding non-user interaction
        }
    }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::B => {
            for x in 0..model.grid.size_x {
                for y in 0..model.grid.size_y {
                    model.grid.get_tile_mut(x,y).tile_type = TileType::Water;
                }
            }
        }
        Key::G => {
            for x in 0..model.grid.size_x {
                for y in 0..model.grid.size_y {
                    model.grid.get_tile_mut(x,y).tile_type = TileType::Ground;
                }
            }
        }
        Key::W => {
            let _ = model.grid.move_cursor(Direction::UP);
        }
        Key::S => {
            let _ = model.grid.move_cursor(Direction::DOWN);
        }
        Key::A => {
            let _ = model.grid.move_cursor(Direction::LEFT);
        }
        Key::D => {
            let _ = model.grid.move_cursor(Direction::RIGHT);
        }
        Key::Space => {
            let _ = model.grid.use_cursor();
        }
        _other_key => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(GREY);

    draw_grid(&draw, &model.grid);
    draw_cursor(&draw, &model.grid.cursor);

    draw.to_frame(app, &frame).unwrap();
}

fn translate_grid_to_px(x: i32, y: i32) -> (f32, f32) {
    let px_x = (x as f32 * TILE_SIZE_PX as f32) - (WINDOW_SIZE_PX_X/2) as f32 + TILE_SIZE_PX as f32;
    let px_y = (y as f32 * TILE_SIZE_PX as f32) - (WINDOW_SIZE_PX_Y/2) as f32 + (TILE_SIZE_PX as f32)/2.0 + 2.0;
    (px_x, px_y)
}

fn draw_grid(draw: &Draw, grid: &Grid) {
    for x in 0..grid.size_x{
        for y in 0..grid.size_y { 
            let tile = grid.get_tile(x, y);
            draw_tile(&draw, tile);
            draw_node(&draw, &tile.node);
        }
    }
}

fn draw_tile(draw: &Draw, tile: &Tile) {
    let tile_color = color_from_tile_type(&tile.tile_type);
    draw_square(&draw, translate_grid_to_px(tile.pos_x, tile.pos_y), TILE_SIZE_PX, tile_color);
}

fn draw_node(draw: &Draw, node: &Option<Node>) {
    match node {
        Some(node) => {
            let node_color = color_from_node_type(&node.node_type);
            match node.node_type {
                NodeType::Bush => draw_circle(&draw, translate_grid_to_px(node.pos_x, node.pos_y), TILE_SIZE_PX-15, node_color),
                _ => draw_square(&draw, translate_grid_to_px(node.pos_x, node.pos_y), TILE_SIZE_PX-15, node_color),
            }
        },
        None => ()
    } 
}

fn draw_cursor(draw: &Draw, cursor: &Option<Cursor>) {
    match cursor {
        Some(cursor) => {
            let cursor_color = color_from_cursor(&cursor);
            draw_unfilled_circle(&draw, translate_grid_to_px(cursor.pos_x, cursor.pos_y), TILE_SIZE_PX, cursor_color);
            draw_node(&draw, &cursor.node);
        }
        None => ()
    }
}

fn draw_square(draw: &Draw, point: (f32, f32), square_size_px: u32, color: Srgb<u8>) {
    draw.rect()
        .w_h(square_size_px as f32 - 1.0, square_size_px as f32 - 1.0)
        .x_y(point.0, point.1)
        .color(color);
}

fn draw_circle(draw: &Draw, point: (f32, f32), diameter_px: u32, color: Srgb<u8>) {
    draw.ellipse()
        .w_h(diameter_px as f32 - 1.0, diameter_px as f32 - 1.0)
        .x_y(point.0, point.1)
        .color(color);
}

fn draw_unfilled_circle(draw: &Draw, point: (f32, f32), diameter_px: u32, color: Srgb<u8>) {
    let radius = (diameter_px - 2) as f32 / 2 as f32;                   
    let points = (0..=360).map(|i| {      
       let radian = deg_to_rad(i as f32); 
       let x = radian.sin() * radius + point.0;     
       let y = radian.cos() * radius + point.1;     
       (pt2(x, y), color)              
    });
    draw.polyline()                       
        .weight(3.0)
        .points_colored(points);  
}

fn color_from_tile_type(tile_type: &TileType) -> Srgb<u8> {
    match tile_type {
        TileType::None => return BLACK,
        TileType::Ground => return TAN,
        TileType::Water => return BLUE,
        TileType::Gold=> return GOLD,
    } 
}

fn color_from_node_type(node_type: &NodeType) -> Srgb<u8> {
    match node_type {
        NodeType::None => return BLACK,//delet?
        NodeType::Rock=> return DARKGREY,
        NodeType::Bush=> return GREEN,
    } 
}

fn color_from_cursor(cursor: &Cursor) -> Srgb<u8> {
    match cursor.node {
        None => return RED,
        Some(_) => return GOLD,
    } 
}


