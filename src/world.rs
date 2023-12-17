//! this will be the module that creates and builds the world that the game takes place in.
//! this will include the voxels and the other world tools

//! first will be the voxel grid
use bevy::prelude::*;
use space_editor::prelude::*;

// #[derive(Component)]
// pub struct WorldSize {
//     /// must be greater than 1
//     length: usize,
//     /// must be greater than 1
//     width: usize,
//     /// must be greater than 0
//     height: usize,
// }

// #[derive(Component)]
// pub struct Cell {
//     loc: Coord,
// }

// pub struct Coord {
//     // index in each dimension
//     length: usize,
//     width: usize,
//     height: usize,
// }
// impl Coord {
//     fn new(length: usize, width: usize, height: usize) -> Self {
//         Coord {
//             length,
//             width,
//             height,
//         }
//     }
// }

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct LevelLayer(pub isize);

pub fn create_level_layout(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // let mut colors = [Color::WHITE, Color::RED, Color::GREEN, Color::BLUE]
    //     .iter()
    //     .cycle();

    commands
        .spawn(SpatialBundle::default())
        .insert(Name::new("Level"))
        .insert(PrefabMarker)
        .with_children(|parent| {
            parent
                .spawn(SpatialBundle::default())
                .insert(Name::new("Layer"))
                .insert(PrefabMarker)
                .insert(LevelLayer(0))
                .with_children(|parent| {
                    for i in 0..10 {
                        for j in 0..10 {
                            parent.spawn((
                                PbrBundle {
                                    mesh: meshes.add(MeshPrimitivePrefab::Cube(1.0).to_mesh()),
                                    material: materials.add(Color::WHITE.into()),
                                    transform: Transform::from_xyz(
                                        5.0 - 0.5 - i as f32,
                                        -0.5,
                                        5.0 - 0.5 - j as f32,
                                    ),
                                    ..default()
                                },
                                PrefabMarker,
                            ));
                        }
                    }
                });
        });
}

pub fn outline_world_grid(mut gizmos: Gizmos) {
    // this is a function that will show the outlines of each cell in the overall grid
    // gizmos.cuboid(, )
}
