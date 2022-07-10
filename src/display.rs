use nannou::prelude::*;
use crate::Node;
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
            
struct Model {
    grid: Grid,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(WINDOW_SIZE_PX_X, WINDOW_SIZE_PX_Y)
        .view(view)
        .build()
        .unwrap();

    let mut grid = Grid::new(GRID_SIZE_X, GRID_SIZE_Y); 
    grid.add_tile_types();
    Model { grid }
}

fn update(app: &App, model: &mut Model, _update: Update) {
println!("UWU");
//    for i in 0..NO_BOIDS {
        // model.boids[i].update();
        // model.boids[i].edge(screen_top, screen_right);
 //   }
    for x in 0..model.grid.size_x{
        for y in 0..model.grid.size_y {
            // TODO figure out clock system
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(GREY);

    // Fill tiles (draw as squares?)
    for x in 0..model.grid.size_x{
        for y in 0..model.grid.size_y {
            draw_tile(&draw, translate_grid_to_px(x, y), TILE_SIZE_PX, &model.grid.tiles[x as usize][y as usize].tile_type);
        }
    }

    // Draw nodes
    draw.to_frame(app, &frame).unwrap();
}

fn translate_grid_to_px(x: i32, y: i32) -> (f32, f32) {
    let px_x = (x as f32 * TILE_SIZE_PX as f32) - (WINDOW_SIZE_PX_X/2) as f32 + TILE_SIZE_PX as f32;
    let px_y = (y as f32 * TILE_SIZE_PX as f32) - (WINDOW_SIZE_PX_Y/2) as f32 + (TILE_SIZE_PX as f32)/2.0 + 2.0;
    (px_x, px_y)
}

fn draw_tile(draw: &Draw, point: (f32, f32), tile_size_px: u32, tile_type: &TileType) {
    draw.rect()
        .w_h(tile_size_px as f32 - 1.0, tile_size_px as f32 - 1.0)
        .x_y(point.0, point.1)
        .color(color_from_tile_type(tile_type));
}

fn color_from_tile_type( tile_type: &TileType) -> Srgb<u8> {
    match tile_type {
        TileType::None => return TAN,
        TileType::Ground => return BROWN,
        TileType::Water => return BLUE,
    } 
}













