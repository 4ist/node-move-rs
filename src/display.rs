use nannou::prelude::*;
use crate::Node;
use crate::NodeType;
use crate::Grid;
use crate::Tile;
use crate::TileType;

pub fn display() {
    nannou::app(model).update(update).run();
}


const WINDOW_SIZE_PX_X: u32 = 1500;
const WINDOW_SIZE_PX_Y: u32 = 900;
const TILE_SIZE_PX: u32 = 32;

const GRID_SIZE_PX_X: u32 = (WINDOW_SIZE_PX_X - (2 * TILE_SIZE_PX)) as u32;
const GRID_SIZE_PX_Y: u32 = (WINDOW_SIZE_PX_Y - (2 * TILE_SIZE_PX)) as u32;

const GRID_SIZE_X: i32 = (WINDOW_SIZE_PX_X / TILE_SIZE_PX) as i32;
const GRID_SIZE_Y: i32 = (WINDOW_SIZE_PX_Y / TILE_SIZE_PX) as i32;

// TODO move direction vectors to an enum or something
const UP: (i32, i32) = (0, 1);
const DOWN: (i32, i32) = (0,-1); 
const LEFT: (i32, i32) = (-1,0); 
const RIGHT: (i32, i32) = (1,0); 

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
    grid.setup_add_nodes();
    Model { grid }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    for x in 0..model.grid.size_x{
        for y in 0..model.grid.size_y {
            // TODO figure out clock system
        }
    }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        // Pathfinding mode
        // - Mechanics
        //   - make a random node a target
        //   - make N random nodes huters
        //   - hunters pathfind to the target, first to reach it turns gold or something idk
        // - Mechanisms
        //   - event loop (objects have a structure track clock cycles til event) 
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
            move_cursor(model, UP);
        }
        Key::S => {
            move_cursor(model, DOWN);
        }
        Key::A => {
            move_cursor(model, LEFT);
        }
        Key::D => {
            move_cursor(model, RIGHT);
        }
        _other_key => {}
    }
}

fn move_cursor(model: &mut Model, move_diff: (i32, i32)) {
    match model.grid.cursor_pos {
        Some(cursor_pos) => {
            match model.grid.get_tile(cursor_pos.0, cursor_pos.1).node {
                Some(_) => {
                    let src_pos = (cursor_pos.0, cursor_pos.1);
                    let dst_pos = (src_pos.0 + move_diff.0, src_pos.1 + move_diff.1);
                    if let Ok(_) = model.grid.move_node(src_pos, dst_pos){
                        model.grid.cursor_pos = Some(dst_pos);
                    }
                },
                None => ()
            }
        },
        None => ()
    }
}


fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(GREY);

    for x in 0..model.grid.size_x{
        for y in 0..model.grid.size_y {
            // Tiles
            let tile = model.grid.get_tile(x,y);
            let tile_color = color_from_tile_type(&tile.tile_type);
            draw_square(&draw, translate_grid_to_px(x, y), TILE_SIZE_PX, tile_color);

            // Nodes
            match &tile.node {
                Some(node) => {
                    let node_color = color_from_node_type(&node.node_type);
                    match node.node_type {
                        NodeType::Bush => draw_circle(&draw, translate_grid_to_px(x, y), TILE_SIZE_PX-15, node_color),
                        NodeType::Cursor=> draw_unfilled_circle(&draw, translate_grid_to_px(x, y), TILE_SIZE_PX, node_color),
                        _ => draw_square(&draw, translate_grid_to_px(x, y), TILE_SIZE_PX-15, node_color),
                    }
                },
                None => ()
            }
        }
    }
    draw.to_frame(app, &frame).unwrap();
}

fn translate_grid_to_px(x: i32, y: i32) -> (f32, f32) {
    let px_x = (x as f32 * TILE_SIZE_PX as f32) - (WINDOW_SIZE_PX_X/2) as f32 + TILE_SIZE_PX as f32;
    let px_y = (y as f32 * TILE_SIZE_PX as f32) - (WINDOW_SIZE_PX_Y/2) as f32 + (TILE_SIZE_PX as f32)/2.0 + 2.0;
    (px_x, px_y)
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

   // draw.ellipse()
   //     .w_h(square_size_px as f32 - 1.0, square_size_px as f32 - 1.0)
   //     .x_y(point.0, point.1)
   //     .color(color);
}

fn color_from_tile_type(tile_type: &TileType) -> Srgb<u8> {
    match tile_type {
        TileType::None => return BLACK,
        TileType::Ground => return TAN,
        TileType::Water => return BLUE,
    } 
}

fn color_from_node_type(node_type: &NodeType) -> Srgb<u8> {
    match node_type {
        NodeType::None => return BLACK,//delet?
        NodeType::Rock=> return DARKGREY,
        NodeType::Bush=> return GREEN,
        NodeType::Cursor=> return RED,
    } 
}













