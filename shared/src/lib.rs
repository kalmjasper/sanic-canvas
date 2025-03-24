use bevy::{asset::AssetMetaCheck, prelude::*, window::PrimaryWindow};
use noise::NoiseFn;

use bevy::{
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin},
};

#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct SquareMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[uniform(1)]
    outline_color: LinearRgba,
}

// #[derive(Clone, Copy, ShaderType, TypePath, AsBindGroup)]
// #[repr(C)]
// pub struct Outline {
//     thickness1: f32,
//     thickness2: f32,
//     thickness3: f32,
//     thickness4: f32,
// }

impl Material2d for SquareMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/square.wgsl".into()
    }
}

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
    z: f32,
}

#[derive(Component)]
struct Visual;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn update_points(
    mut points: Query<(&mut Point, &mut Transform, &MeshMaterial2d<SquareMaterial>)>,
    mut materials: ResMut<Assets<SquareMaterial>>,
    time: Res<Time>,
) {
    for (mut point, mut transform, material) in points.iter_mut() {
        point.z = noise::OpenSimplex::new(1).get([
            point.x as f64 / 350.0,
            point.y as f64 / 350.0,
            time.elapsed_secs() as f64 / 1.0,
        ]) as f32
            + 0.5;

        let vec_to_center_len = (point.x.powi(2) + point.y.powi(2)).sqrt();

        let scale = point.z * 0.5 + 0.5;
        transform.scale = Vec3::new(scale, scale, 1.0);
        transform.translation = Vec3::new(
            point.x * (1.0 - (vec_to_center_len / 9000.0) * (1.0 - point.z)),
            point.y * (1.0 - (vec_to_center_len / 9000.0) * (1.0 - point.z)),
            0.0,
        );

        if let Some(material) = materials.get_mut(material.id()) {
            material.color = Color::hsl(
                (1.0 - point.z) * 360.0, // hue
                1.0,                     // saturation
                0.5,                     // lightness
            )
            .into();
        }
    }
}

fn make_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<SquareMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let num_squares_horizontal = 40;
    let white_space_relative_to_square = 0.3;

    let window = window_query.single();

    let width = window.width();
    let height = window.height();

    let square_size_with_whitespace =
        width / (num_squares_horizontal as f32 + white_space_relative_to_square);

    let squares_vertical = (height / square_size_with_whitespace) as i32;
    let side_spacing_y = height % square_size_with_whitespace;

    for x in 0..(num_squares_horizontal + 10) {
        for y in 0..(squares_vertical + 10) {
            let spacing = square_size_with_whitespace * white_space_relative_to_square;
            let square_size = square_size_with_whitespace - spacing;

            let pos_x = (spacing + square_size_with_whitespace) / 2.0
                + (x as f32 * square_size_with_whitespace)
                - width / 2.0
                - square_size_with_whitespace * 5.0;

            let pos_y = (side_spacing_y + square_size_with_whitespace) / 2.0
                + (y as f32 * square_size_with_whitespace)
                - height / 2.0
                - square_size_with_whitespace * 5.0;

            commands.spawn((
                Point {
                    x: pos_x,
                    y: pos_y,
                    z: 0.0,
                },
                Mesh2d(meshes.add(Rectangle::new(square_size, square_size))),
                MeshMaterial2d(materials.add(SquareMaterial {
                    color: LinearRgba::WHITE,
                    outline_color: LinearRgba::BLACK,
                })),
                Transform::from_xyz(pos_x, pos_y, 0.0).with_scale(Vec3::new(0.0, 0.0, 1.0)),
                Visual,
            ));
        }
    }
}

pub async fn run_app(canvas_id: Option<&str>) {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    canvas: canvas_id.map(|id| id.to_string()),
                    fit_canvas_to_parent: true,
                    resize_constraints: WindowResizeConstraints {
                        min_width: 0.0,
                        min_height: 0.0,
                        max_width: f32::INFINITY,
                        max_height: f32::INFINITY,
                    },
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            }),
        Material2dPlugin::<SquareMaterial>::default(),
    ))
    .insert_resource(ClearColor(Color::BLACK))
    .insert_resource(WindowSize::default())
    .add_systems(Startup, setup)
    .add_systems(
        Update,
        (update_window_size, (cleanup_visuals, make_grid).chain()).run_if(detect_window_resize),
    )
    .add_systems(Update, update_points);

    app.run();
}
