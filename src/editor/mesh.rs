use bevy::prelude::*;
use space_editor::prelude::*;

pub struct EditorMeshPlugin;

impl Plugin for EditorMeshPlugin {
    fn build(&self, app: &mut App) {
        app.editor_bundle(
            "Mesh",
            "Layer",
            (
            // this is a bundle of n x m MeshPrimitivePrefab Cubes with some other data
        ),
        );
    }
}
