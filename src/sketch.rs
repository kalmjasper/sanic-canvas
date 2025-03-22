// Derived from the example in nature_of_code and noise visualization

// The Nature of Code
// Daniel Shiffman
// http://natureofcode.com
//
// Example 8-3: Simple Recursion

use std::cell::RefCell;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};

// struct Particle {
//     pos: Vector2,
//     vel: Vector2,
// }
//
// impl Particle {
//     fn new(x: f32, y: f32) -> Particle {
//         Particle {
//             pos: vec2(x, y),
//             vel: vec2(0., 0.),
//         }
//     }
//
//     fn update(&mut self, dir: Vector2) {
//         self.pos += self.vel;
//         self.vel += dir;
//         self.vel *= 0.8;
//     }
// }
//
// struct Model {
//     points: Vec<Vector3>,
//     noise: nannou::noise::OpenSimplex,
// }
//
// fn model(app: &App) -> Model {
//     let mut p = vec![];
//     for x in -20..20 {
//         for y in -20..20 {
//             p.push(vec3(2.0 * x as f32, 2.0 * y as f32, 0.0));
//         }
//     }
//     let noise = nannou::noise::OpenSimplex::new();
//     Model { points: p, noise }
// }
//
// fn update(app: &App, model: &mut Model) {
//     let t = (app.elapsed_frames() as f32) * 0.015;
//     let mut pn = vec![];
//
//     for p in &model.points {
//         let r = model
//             .noise
//             .get([p.x as f64 / 10.0, p.y as f64 / 10.0, t as f64]);
//         pn.push(vec3(p.x, p.y, r as f32));
//     }
//     model.points = pn;
// }
//
// fn view(app: &App, model: &Model) {
//     let draw = app.draw();
//     draw.background().color(BLACK);
//
//     for point in &model.points {
//         let d = vec2(point.x, point.y).normalize();
//         let r = point.z * 15.0 + 12.0;
//         let p = vec2(point.x, point.y) * 15.0 + d * point.z * 15.0;
//         draw.rect()
//             .x_y(p.x, p.y)
//             .w_h(r, r)
//             .color(DARK_GRAY)
//             .stroke(Color::hsl(1.0 - point.z as f32 / 2.0 + 0.5, 1.0, 0.5))
//             .stroke_weight(2.0 - point.z);
//     }
//     // draw.to_frame(app).unwrap();
// }

pub async fn run_app() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            canvas: Some("#sanic".into()),
            fit_canvas_to_parent: true,

            ..default()
        }),
        ..default()
    }));
    app.run();
}

// fn create_window(app: &App) {
//     app.new_window().primary().view(view).build();
//
//     // app.new_window().title("nannou web test").view(view).build();
// }
