use bevy::prelude::*;

pub mod mesh;
use mesh::*;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EditorMeshPlugin);
    }
}
