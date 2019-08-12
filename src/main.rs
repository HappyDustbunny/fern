extern crate rand;

use nannou::prelude::*;
use rand::Rng;

fn main() {
    nannou::app(model).run()
}

struct Model {
}

fn model(app: &App) -> Model {
    let window = app
    .new_window()
    .with_dimensions(500, 800)
    .with_title("Fern")
    .view(view)
    .build()
    .unwrap();
}

fn view(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    let mut point = Vector2::new(0.0, 0.0);
    let mut new_point = Vector2::new(0.0, 0.0);

    let mut rng = rand::thread_rng();

    let choise: i8 = rng.gen();

    if choise == 1  {
        new_point[0] = 0.0 * point[0];
        new_point[1] = 0.16 * point[1];
    } else if choise == 2 {
        new_point[0] = 0.2125 * (point[0] - point[1]);
        new_point[1] = 0.2125 * (point[0] + point[1]) + 0.64;
    } else if choise == 3 {
        new_point[0] = 0.2375 * (-point[0] + point[1]);
        new_point[1] = 0.2125 * (point[0] + point[1]) + 0.176;
    } else {
        new_point[0] = 0.85 * point[0];
        new_point[1] = 0.85 * point[1] + 0.64;
    }

    draw.ellipse().x_y(point[0], point[1]).color(WHITE);

    draw.to_frame(app, frame).unwrap();
}
