use bevy::{prelude::shape, prelude::*, render::camera::ScalingMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::{backends::raycast::RaycastPickable, prelude::*};
use space_editor::prelude::*;

mod camera;
mod editor;
mod world;

use camera::*;
use editor::EditorPlugin as LocalEditorPlugin;
use world::create_level_layout;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LocalEditorPlugin)
        // .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(SpaceEditorPlugin)
        .add_systems(Startup, create_level_layout)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (show_origin, zoom_camera, move_camera, rotate_camera),
        )
        .run();
}

#[derive(Component)]
pub struct GameCamera;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut config: ResMut<GizmoConfig>,
) {
    config.depth_bias = -1.;
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
        GameCamera,
        PrefabMarker,
    ));

    // light
    commands
        .spawn(PointLightBundle {
            transform: Transform::from_xyz(3.0, 8.0, 5.0),
            ..default()
        })
        .insert(PrefabMarker);

    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(bevy_panorbit_camera::PanOrbitCamera::default())
        .insert(EditorCameraMarker)
        .insert(PickableBundle::default())
        .insert(RaycastPickable);

    bevy_debug_grid::spawn_floor_grid(commands);
}

fn show_origin(mut gizmos: Gizmos) {
    gizmos.ray(Vec3::ZERO, Vec3::X, Color::RED);
    gizmos.ray(Vec3::ZERO, Vec3::Y, Color::GREEN);
    gizmos.ray(Vec3::ZERO, Vec3::Z, Color::BLUE);
}
