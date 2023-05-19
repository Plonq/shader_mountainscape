use crate::material::LandMaterial;
use bevy::pbr::wireframe::{Wireframe, WireframePlugin};
use bevy::prelude::*;
use bevy::render::mesh::VertexAttributeValues;
use bevy::render::settings::{WgpuFeatures, WgpuSettings};
use bevy::render::RenderPlugin;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_shader_utils::ShaderUtilsPlugin;
use itertools::Itertools;
use material::ShipMaterial;
use std::f32::consts::TAU;

mod material;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::DARK_GRAY))
        .add_plugins(DefaultPlugins.set(RenderPlugin {
            wgpu_settings: WgpuSettings {
                features: WgpuFeatures::POLYGON_MODE_LINE,
                ..default()
            },
        }))
        .add_plugin(MaterialPlugin::<ShipMaterial>::default())
        .add_plugin(MaterialPlugin::<LandMaterial>::default())
        .add_plugin(ShaderUtilsPlugin)
        .add_plugin(PanOrbitCameraPlugin)
        .add_plugin(WireframePlugin)
        .add_startup_system(setup)
        .add_systems((move_cube, move_land))
        .run();
}

#[derive(Component)]
struct Land;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut ship_materials: ResMut<Assets<ShipMaterial>>,
    mut land_materials: ResMut<Assets<LandMaterial>>,
) {
    // land
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: 600.0,
                subdivisions: 2000,
            })),
            transform: Transform::from_xyz(0.0, 0.0, -300.0),
            material: land_materials.add(LandMaterial {
                color: Color::GREEN,
                alpha_mode: AlphaMode::Opaque,
            }),
            ..default()
        },
        // Wireframe,
        Land,
    ));

    // ship
    let mut mesh = Mesh::from(shape::Cube { size: 1.0 });
    if let Some(VertexAttributeValues::Float32x3(positions)) =
        mesh.attribute(Mesh::ATTRIBUTE_POSITION)
    {
        let colors: Vec<[f32; 4]> = positions
            .iter()
            .map(|[r, g, b]| [(1. - *r) / 2., (1. - *g) / 2., (1. - *b) / 2., 1.])
            .collect();
        // mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    }
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(mesh),
            transform: Transform::from_xyz(0.0, 1.0, -5.0),
            material: ship_materials.add(ShipMaterial {
                color: Color::RED,
                alpha_mode: AlphaMode::Opaque,
            }),
            ..default()
        },
        Movable,
    ));

    // camera
    commands.spawn((
        Camera3dBundle {
            // transform: Transform::from_xyz(0.0, 1.0, 1.0).looking_at(Vec3::Y, Vec3::Y),
            ..default()
        },
        PanOrbitCamera {
            radius: 1.0,
            focus: Vec3::Y,
            ..default()
        },
    ));
}

#[derive(Component)]
struct Movable;

fn move_cube(
    time: Res<Time>,
    key_input: Res<Input<KeyCode>>,
    mut cubes_q: Query<&mut Transform, With<Movable>>,
) {
    let mut velocity = Vec3::ZERO;
    let mut rotation_y = 0.0;
    let mut rotation_x = 0.0;

    if key_input.pressed(KeyCode::W) {
        velocity.z -= 1.;
    }
    if key_input.pressed(KeyCode::S) {
        velocity.z += 1.;
    }
    if key_input.pressed(KeyCode::A) {
        velocity.x -= 1.;
    }
    if key_input.pressed(KeyCode::D) {
        velocity.x += 1.;
    }
    if key_input.pressed(KeyCode::Space) {
        velocity.y += 1.;
    }
    if key_input.pressed(KeyCode::LShift) {
        velocity.y -= 1.;
    }

    if key_input.pressed(KeyCode::Left) {
        rotation_y = -TAU * 0.125;
    }
    if key_input.pressed(KeyCode::Right) {
        rotation_y = TAU * 0.125;
    }
    if key_input.pressed(KeyCode::Up) {
        rotation_x = -TAU * 0.125;
    }
    if key_input.pressed(KeyCode::Down) {
        rotation_x = TAU * 0.125;
    }

    for mut cube_tfm in cubes_q.iter_mut() {
        cube_tfm.translation += velocity.normalize_or_zero() * time.delta_seconds() * 2.0;
        cube_tfm.rotate_y(rotation_y * time.delta_seconds());
        cube_tfm.rotate_x(rotation_x * time.delta_seconds());
    }
}

fn move_land(time: Res<Time>, mut land_q: Query<&mut Transform, With<Land>>) {
    for mut land in land_q.iter_mut() {
        land.translation.z += time.delta_seconds();
    }
}
