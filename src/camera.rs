use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, (move_camera, rotate_camera));
    }
}

fn move_camera(
    mut query: Query<&mut Transform, With<Camera>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let velocity = 10.0;
    for mut transform in query.iter_mut() {
        let forward = transform.forward();
        let backwards = transform.back();
        let left = transform.left();
        let right = transform.right();
        let up = transform.up();
        let down = transform.down();
        if input.pressed(KeyCode::Space) {
            transform.translation += velocity * up * time.delta_seconds();
        }
        if input.pressed(KeyCode::ShiftLeft) {
            transform.translation += velocity * down * time.delta_seconds();
        }
        if input.pressed(KeyCode::W) {
            transform.translation += velocity * forward * time.delta_seconds();
        }
        if input.pressed(KeyCode::S) {
            transform.translation += velocity * backwards * time.delta_seconds();
        }
        if input.pressed(KeyCode::A) {
            transform.translation += velocity * left * time.delta_seconds();
        }
        if input.pressed(KeyCode::D) {
            transform.translation += velocity * right * time.delta_seconds();
        }
    }
}

fn rotate_camera(
    mut query: Query<&mut Transform, With<Camera>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let theta = 1.;
    for mut transform in query.iter_mut() {
        if input.pressed(KeyCode::Q) {
            transform.rotate(Quat::from_rotation_y(theta * time.delta_seconds()))
        }
        if input.pressed(KeyCode::E) {
            transform.rotate(Quat::from_rotation_y(-theta * time.delta_seconds()))
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-5.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
