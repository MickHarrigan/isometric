use bevy::{input::mouse::MouseWheel, prelude::*};
use std::f32::consts::PI;

#[derive(Component, Debug)]
pub struct OrthographicFocus(pub Vec3);

pub fn zoom_camera(
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

pub fn rotate_camera(
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

pub fn move_camera(
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
