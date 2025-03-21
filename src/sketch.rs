// Derived from the example in nature_of_code and noise visualization

// The Nature of Code
// Daniel Shiffman
// http://natureofcode.com
//
// Example 8-3: Simple Recursion

use std::cell::RefCell;

use nannou::prelude::*;

pub struct Model {
    // We can remove the previous fields as they're no longer needed
}

fn update(_app: &App, _model: &mut Model) {
    // No updates needed for this visualization
}

fn view(app: &App, _model: &Model) {
    // Begin drawing
    let draw = app.draw();
    draw.background().color(BLACK);

    let win = app.window_rect();
    let t = app.time();

    // Decide on a number of points and a weight.
    let n_points = 10;
    let weight = 8.0;
    let hz = ((app.mouse().x + win.right()) / win.w()).powi(4) * 1000.0;
    let points_colored = (0..n_points)
        // A sine wave mapped to the range of the window.
        .map(|i| {
            let x = map_range(i, 0, n_points - 1, win.left(), win.right());
            let fract = i as f32 / n_points as f32;
            let amp = (t + fract * hz * TAU).sin();
            let y = map_range(amp, -1.0, 1.0, win.bottom() * 0.75, win.top() * 0.75);
            pt2(x, y)
        })
        .enumerate()
        // Colour each vertex uniquely based on its index.
        .map(|(i, p)| {
            let fract = i as f32 / n_points as f32;
            let r = (t + fract) % 1.0;
            let g = (t + 1.0 - fract) % 1.0;
            let b = (t + 0.5 + fract) % 1.0;
            let rgba = Color::srgba(r, g, b, 1.0);
            (p, rgba)
        });

    // Draw the polyline as a stroked path.
    draw.polyline()
        .weight(weight)
        .join_round()
        .points_colored(points_colored);
}

pub async fn run_app(_model: Model) {
    thread_local!(static MODEL: RefCell<Option<Model>> = Default::default());

    let model = Model {}; // Simplified model with no fields

    MODEL.with(|m| m.borrow_mut().replace(model));

    nannou::app(|app| {
        create_window(app);
        Model {} // Simplified model initialization
    })
    .update(update)
    .run();
}

fn create_window(app: &App) {
    app.new_window().primary().view(view).build();

    // app.new_window().title("nannou web test").view(view).build();
}
