use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use std::f32::consts::TAU;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin::<CustomMaterial>::default())
        .add_startup_system(setup)
        .add_systems((move_cube,))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    // cube
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            material: materials.add(CustomMaterial {
                alpha_mode: AlphaMode::Blend,
            }),
            ..default()
        },
        Movable,
    ));

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
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

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CustomMaterial {
    alpha_mode: AlphaMode,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for CustomMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/shader_mat.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/shader_mat.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}
