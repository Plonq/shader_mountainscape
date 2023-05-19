use crate::material::LandMaterial;
use bevy::prelude::*;

use bevy::render::settings::{WgpuFeatures, WgpuSettings};
use bevy::render::RenderPlugin;
use bevy_shader_utils::ShaderUtilsPlugin;

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
        .add_plugin(MaterialPlugin::<LandMaterial>::default())
        .add_plugin(ShaderUtilsPlugin)
        .add_startup_system(setup)
        .add_systems((move_land, move_camera))
        .run();
}

#[derive(Component)]
struct Land;

#[derive(Component)]
struct Camera;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut land_materials: ResMut<Assets<LandMaterial>>,
) {
    // land 1
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: 300.0,
                subdivisions: 1500,
            })),
            transform: Transform::from_xyz(0.0, -2.0, -150.0),
            material: land_materials.add(LandMaterial {
                color: Color::GREEN,
                offset: 0.0,
                alpha_mode: AlphaMode::Opaque,
            }),
            ..default()
        },
        Land,
    ));

    // land 2
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: 300.0,
                subdivisions: 1500,
            })),
            transform: Transform::from_xyz(0.0, 2.0, -150.0)
                .with_rotation(Quat::from_rotation_z(180f32.to_radians())),
            material: land_materials.add(LandMaterial {
                color: Color::GREEN,
                offset: 1000.0,
                alpha_mode: AlphaMode::Opaque,
            }),
            ..default()
        },
        Land,
    ));

    // camera
    commands.spawn((Camera3dBundle::default(), Camera));
}

fn move_land(time: Res<Time>, mut land_q: Query<&mut Transform, With<Land>>) {
    for mut land in land_q.iter_mut() {
        land.translation.z += time.delta_seconds();
    }
}

fn move_camera(time: Res<Time>, mut camera_q: Query<&mut Transform, With<Camera>>) {
    for mut camera in camera_q.iter_mut() {
        camera.rotate_z(10f32.to_radians() * time.delta_seconds());
    }
}
