use nannou::prelude::*;
use crate::Node;
use rand::prelude::*;

pub fn display() {
    nannou::app(model).update(update).run();
}

const NO_BOIDS: usize = 50;
const NO_PREDATOR: usize = 2;

const WINDOW_SIZE_X: u32 = 1500;
const WINDOW_SIZE_Y: u32 = 900;

struct Model {
    boids: Vec<Node>,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(WINDOW_SIZE_X, WINDOW_SIZE_Y)
        .view(view)
        .build()
        .unwrap();

    let mut boids: Vec<Node> = Vec::new();

    for i in 0..NO_BOIDS {
        boids.push(Node::new_random(WINDOW_SIZE_X, WINDOW_SIZE_Y));
    }

    Model { boids }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let screen_right = app.window_rect().right() as f32;
    let screen_top = app.window_rect().top() as f32;

    for i in 0..NO_BOIDS {

        // model.boids[i].update();
        // model.boids[i].edge(screen_top, screen_right);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(GREY);

    for i in 0..NO_BOIDS {
        model.boids[i].show(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}
