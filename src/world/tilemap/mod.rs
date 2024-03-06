use crate::world::chunk::ChunkPos;
use bevy::ecs::entity::MapEntities;
use bevy::prelude::*;
use bevy::utils::HashMap;

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {}
}

pub struct TileSettings {
    pub tile_size: f32,
}

#[derive(Resource, Default)]
pub struct TilemapSystem {
    pub chunks: HashMap<ChunkPos, Entity>,
}

impl TilemapSystem {
    pub fn new() -> Self {
        todo!()
    }
}

impl MapEntities for TilemapSystem {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        for chunk in self.chunks.values_mut() {
            *chunk = entity_mapper.map_entity(*chunk);
        }
    }
}
