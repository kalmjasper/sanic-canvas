// Derived from the example in nature_of_code and noise visualization

// The Nature of Code
// Daniel Shiffman
// http://natureofcode.com
//
// Example 8-3: Simple Recursion

use std::cell::RefCell;

use nannou::noise::NoiseFn;
use nannou::prelude::*;
use nannou::wgpu::{Backends, DeviceDescriptor, Limits};

pub struct Model {
    pub points: Vec<Vec3>,
    pub noise: nannou::noise::OpenSimplex,
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let t = (app.elapsed_frames() as f32) * 0.015;
    let mut pn = vec![];

    for p in &model.points {
        let r = model
            .noise
            .get([p.x as f64 / 10.0, p.y as f64 / 10.0, t as f64]);
        pn.push(vec3(p.x, p.y, r as f32));
    }
    model.points = pn;
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();
    draw.background().color(BLACK);

    for point in &model.points {
        let d = vec2(point.x, point.y).normalize();
        let r = point.z * 6.0 + 6.0;
        let p = vec2(point.x, point.y) * 15.0 + d * point.z * 15.0;
        draw.rect()
            .x_y(p.x, p.y)
            .w_h(r, r)
            .color(DARKGREY)
            .stroke(hsl(1.0 - point.z as f32 / 2.0 + 0.5, 1.0, 0.5))
            .stroke_weight(2.0 - point.z);
    }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

pub async fn run_app(_model: Model) {
    // Since ModelFn is not a closure we need this workaround to pass the calculated model
    thread_local!(static MODEL: RefCell<Option<Model>> = Default::default());

    // Create a new model instead of using the passed one
    let mut p = vec![];
    for x in -20..20 {
        for y in -20..20 {
            p.push(vec3(x as f32, y as f32, 0.0));
        }
    }
    let noise = nannou::noise::OpenSimplex::new();
    let model = Model {
        points: p,
        noise: noise,
    };

    MODEL.with(|m| m.borrow_mut().replace(model));

    app::Builder::new_async(|app| {
        Box::new(async move {
            create_window(app).await;
            MODEL.with(|m| m.borrow_mut().take().unwrap())
        })
    })
    .backends(Backends::PRIMARY | Backends::GL)
    .update(update)
    .run_async()
    .await;
}

async fn create_window(app: &App) {
    let device_desc = DeviceDescriptor {
        limits: Limits {
            max_texture_dimension_2d: 8192,
            ..Limits::downlevel_webgl2_defaults()
        },
        ..Default::default()
    };

    app.new_window()
        .device_descriptor(device_desc)
        .title("nannou web test")
        // .raw_event(raw_event)
        // .key_pressed(key_pressed)
        // .key_released(key_released)
        // .mouse_pressed(mouse_pressed)
        // .mouse_moved(mouse_moved)
        // .mouse_released(mouse_released)
        // .mouse_wheel(mouse_wheel)
        // .touch(touch)
        .view(view)
        .build_async()
        .await
        .unwrap();
}
