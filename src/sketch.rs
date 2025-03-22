// Derived from the example in nature_of_code and noise visualization

// The Nature of Code
// Daniel Shiffman
// http://natureofcode.com
//
// Example 8-3: Simple Recursion

use web_sys::console;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};

#[derive(Component)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Visual;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    // window_query: Query<&Window, With<PrimaryWindow>>,
) {
    console::log_1(&"setup".into());

    // let window = window_query.single();

    commands.spawn(Camera2d::default());

    for x in -20..20 {
        for y in -20..20 {
            let base_pos = Vec2::new(20.0 * x as f32, 20.0 * y as f32);

            commands.spawn((
                Point {
                    x: base_pos.x,
                    y: base_pos.y,
                },
                Mesh2d(meshes.add(Rectangle::new(10.0, 10.0))),
                MeshMaterial2d(materials.add(ColorMaterial::from(Color::WHITE))),
                Transform::from_xyz(base_pos.x, base_pos.y, 0.0),
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
    .add_systems(Startup, setup);

    app.run();
}
