use std::{f32::consts::PI, time::Duration};

use bevy::{input::mouse::MouseWheel, prelude::*, render::camera::ScalingMode};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (show_origin, zoom_camera, move_camera, rotate_camera),
        )
        .run();
}

#[derive(Component, Debug)]
struct OrthographicFocus(pub Vec3);

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // camera
    commands.spawn((
        Camera3dBundle {
            projection: OrthographicProjection {
                scale: 5.0,
                scaling_mode: ScalingMode::FixedVertical(2.0),
                ..default()
            }
            .into(),
            transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        OrthographicFocus(Vec3::ZERO),
    ));

    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cubes
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::RED.into()),
        transform: Transform::from_xyz(1.5, 0.5, 1.5),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::GREEN.into()),
        transform: Transform::from_xyz(1.5, 0.5, -1.5),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::BLUE.into()),
        transform: Transform::from_xyz(-1.5, 0.5, 1.5),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::WHITE.into()),
        transform: Transform::from_xyz(-1.5, 0.5, -1.5),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(3.0, 8.0, 5.0),
        ..default()
    });
}

fn show_origin(mut gizmos: Gizmos) {
    gizmos.ray(Vec3::ZERO, Vec3::X, Color::RED);
    gizmos.ray(Vec3::ZERO, Vec3::Y, Color::GREEN);
    gizmos.ray(Vec3::ZERO, Vec3::Z, Color::BLUE);
}

fn zoom_camera(
    mut camera: Query<&mut Projection, With<Camera>>,
    mut mouse: EventReader<MouseWheel>,
) {
    let mut delta_zoom = 0.;
    for ev in mouse.read() {
        delta_zoom += ev.y;
    }
    if delta_zoom == 0. {
        return;
    }

    let Ok(mut cam) = camera.get_single_mut() else {
        return;
    };

    if let Projection::Orthographic(ref mut cam) = *cam {
        cam.scale -= delta_zoom;
        cam.scale = cam.scale.clamp(3.0, 12.0);
    }
}

fn rotate_camera(
    input: Res<Input<KeyCode>>,
    mut camera: Query<(&mut Transform, &mut OrthographicFocus), With<Camera>>,
) {
    let Ok((mut cam, focus)) = camera.get_single_mut() else {
        return;
    };
    let mut angle: Option<f32> = None;
    if input.just_pressed(KeyCode::Q) {
        angle = Some(-PI / 2.);
    } else if input.just_pressed(KeyCode::F) {
        angle = Some(PI / 2.);
    }
    match angle {
        None => return,
        Some(angle) => {
            let rot = Mat3::from_axis_angle(Vec3::Y, angle);

            let adjusted_translation = cam.translation - focus.0;
            let rotation = rot.mul_vec3(adjusted_translation);
            cam.translation = rotation + focus.0;
            cam.look_at(focus.0, Vec3::Y);
        }
    }
}

fn move_camera(
    mut camera: Query<(&mut Transform, &mut Projection, &mut OrthographicFocus), With<Camera>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let Ok((mut cam_tran, mut cam_proj, mut focus)) = camera.get_single_mut() else {
        return;
    };

    if input.just_pressed(KeyCode::Space) {
        *cam_tran = Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y);
        focus.0 = Vec3::ZERO;
        if let Projection::Orthographic(ref mut cam) = *cam_proj {
            cam.scale = 5.0;
        }
    }

    let speed = 5.0;

    let mut movement = Vec3::ZERO;
    if input.pressed(KeyCode::W) {
        let disp = cam_tran.forward().normalize().xz();
        movement.x += disp.x;
        movement.z += disp.y;
    }
    if input.pressed(KeyCode::R) {
        let disp = cam_tran.back().normalize().xz();
        movement.x += disp.x;
        movement.z += disp.y;
    }
    if input.pressed(KeyCode::A) {
        movement += cam_tran.left().normalize();
    }
    if input.pressed(KeyCode::S) {
        movement += cam_tran.right().normalize();
    }

    cam_tran.translation += movement.normalize_or_zero() * time.delta_seconds() * speed;
    focus.0 += movement.normalize_or_zero() * time.delta_seconds() * speed;
}
