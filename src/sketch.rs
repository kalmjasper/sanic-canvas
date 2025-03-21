// Derived from the example in nature_of_code and noise visualization

// The Nature of Code
// Daniel Shiffman
// http://natureofcode.com
//
// Example 8-3: Simple Recursion

use std::cell::RefCell;

use nannou::noise::NoiseFn;
use nannou::prelude::*;
use nannou::rand::*;

struct Particle {
    pos: Vector2,
    vel: Vector2,
}

impl Particle {
    fn new(x: f32, y: f32) -> Particle {
        Particle {
            pos: vec2(x, y),
            vel: vec2(0., 0.),
        }
    }

    fn update(&mut self, dir: Vector2) {
        self.pos += self.vel;
        self.vel += dir;
        self.vel *= 0.8;
    }
}

struct Model {
    particles: Vec<Particle>,
}

fn model(app: &App) -> Model {
    let r = app.window_rect().right() as f32;
    let l = app.window_rect().left() as f32;

    let w = l - r;
    let t = app.window_rect().top() as f32;
    let b = app.window_rect().bottom() as f32;

    let h = t - b;

    let mut p = vec![];
    for _i in 0..200 {
        let x = random_f32() * w + r;
        let y = random_f32() * h + b;
        p.push(Particle::new(x, y));
    }

    Model { particles: p }
}

fn update(app: &App, model: &mut Model) {
    let noise = nannou::noise::Perlin::new();
    let t = app.elapsed_frames() as f64 / 10.;
    for i in 0..model.particles.len() {
        let p = &mut model.particles[i];
        let x = noise.get([
            p.pos.x as f64 / 128.,
            p.pos.y as f64 / 137.,
            t + i as f64 / 1000.,
        ]);
        let y = noise.get([
            -p.pos.y as f64 / 128.,
            p.pos.x as f64 / 137.,
            t + i as f64 / 1000.,
        ]);

        let a = vec2(x as f32, y as f32);
        p.update(a);
    }
}

fn view(app: &App, model: &Model) {
    let draw = app.draw();
    let t = (app.elapsed_frames() as f32) * 0.02;
    let w = (t * 0.832).cos();
    for p in &model.particles {
        draw.ellipse()
            .xy(p.pos)
            .w_h(2.0, 2.0)
            .color(Color::hsla(0.1, 1. + w, 5., 0.01));
    }
    // draw.to_frame(app, &frame).unwrap();
}

pub async fn run_app() {
    nannou::app(|app| {
        create_window(app);
        model(app)
    })
    .update(update)
    .run();
}

fn create_window(app: &App) {
    app.new_window().primary().view(view).build();

    // app.new_window().title("nannou web test").view(view).build();
}
