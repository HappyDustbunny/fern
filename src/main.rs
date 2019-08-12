extern crate rand;

use nannou::prelude::*;
use rand::Rng;

fn main() {
    nannou::app(model).run()
}

struct Model {
    _window: WindowId,
}

fn model(app: &App) -> Model {
    let _window = app
    .new_window()
    .with_dimensions(500, 800)
    .with_title("Fern")
    .view(view)
    .build()
    .unwrap();
    Model { _window }
}

fn view(app: &App, _model: &Model, frame: &Frame) {
    let v1 = 0.855252;
    let v3 = -0.0436;
    let draw = app.draw();

    // draw.background().color(BLACK);

    let mut point = Vector2::new(0.0, 0.0);
    let mut new_point = Vector2::new(0.0, 0.0);

    let mut rng = rand::thread_rng();

    for _ in 1..50000 {

        let choise: i8 = rng.gen_range(1,11);

        if choise == 1  {
            new_point[0] = 0.0 * point[0];
            new_point[1] = 0.16 * point[1];
        } else if choise == 2 {
            new_point[0] = 0.2125 * 2.0.sqrt() * (v1.cos() * point[0] - v1.sin() * point[1]);
            new_point[1] = 0.2125 * 2.0.sqrt() * (v1.sin() * point[0] + v1.cos() * point[1]) + 0.64;
        } else if choise == 3 {
            new_point[0] = 0.2375 * (-point[0] + point[1]);
            new_point[1] = 0.2375 * (point[0] + point[1]) + 0.176;
        } else {
            new_point[0] = 0.85 * (v3.cos() * point[0] - v3.sin() * point[1]);
            new_point[1] = 0.85 * (v3.sin() * point[0] + v3.cos() * point[1]) + 0.64;
        }

        point = new_point;

        draw.ellipse().x_y(150.0*point[0], 150.0*point[1] - 300.0).radius(1.0).color(WHITE);
    }

    draw.to_frame(app, frame).unwrap();
}
