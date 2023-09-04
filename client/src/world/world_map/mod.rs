use bevy::prelude::*;

pub mod chunk;
mod chunk_manager;
mod world_map;

pub use world_map::WorldMap;

pub struct WorldMapPlugin;
impl Plugin for WorldMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(chunk_manager::ChunkManagerPlugin)
            .add_plugin(chunk::ChunkPlugin)
            .init_resource::<WorldMap>();
    }
}
