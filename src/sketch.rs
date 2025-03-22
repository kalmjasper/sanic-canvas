// Derived from the example in nature_of_code and noise visualization

// The Nature of Code
// Daniel Shiffman
// http://natureofcode.com
//
// Example 8-3: Simple Recursion

use web_sys::console;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};

fn cleanup_visuals(mut commands: Commands, query: Query<Entity, With<Visual>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

#[derive(Resource, Default)]
struct WindowSize {
    width: f32,
    height: f32,
}

// Add this system to detect window resizes
fn update_window_size(
    mut window_size: ResMut<WindowSize>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();

    window_size.width = window.width();
    window_size.height = window.height();
}

fn detect_window_resize(
    window_size: Res<WindowSize>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) -> bool {
    let window = window_query.single();
    window.width() != window_size.width || window.height() != window_size.height
}

#[derive(Component)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Visual;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn make_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let num_squares_horizontal = 20;
    let white_space_relative_to_square = 0.3;

    let window = window_query.single();

    let width = window.width();
    let height = window.height();

    let square_size_with_whitespace =
        width / (num_squares_horizontal as f32 + white_space_relative_to_square);

    let squares_vertical = (height / square_size_with_whitespace) as i32;

    console::log_1(&"setup".into());

    console::log_1(&format!("Window size: {}x{}", width, height).into());

    for x in 0..num_squares_horizontal {
        for y in 0..squares_vertical {
            // Calculate position with proper spacing
            let spacing = square_size_with_whitespace * white_space_relative_to_square;
            let square_size = square_size_with_whitespace - spacing;

            // Center the grid in the window
            let pos_x = (spacing + square_size_with_whitespace) / 2.0
                + (x as f32 * square_size_with_whitespace)
                - width / 2.0;

            let pos_y = 0.0;

            commands.spawn((
                Point { x: pos_x, y: pos_y },
                Mesh2d(meshes.add(Rectangle::new(square_size, square_size))),
                MeshMaterial2d(materials.add(ColorMaterial::from(Color::WHITE))),
                Transform::from_xyz(pos_x, pos_y, 0.0),
                Visual,
            ));
        }
    }
}

pub async fn run_app() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            canvas: Some("#sanic".into()),
            fit_canvas_to_parent: true,

            ..default()
        }),
        ..default()
    }))
    .insert_resource(ClearColor(Color::BLACK))
    .insert_resource(WindowSize::default())
    .add_systems(Startup, setup)
    .add_systems(
        Update,
        (update_window_size, (cleanup_visuals, make_grid).chain()).run_if(detect_window_resize),
    );

    app.run();
}
