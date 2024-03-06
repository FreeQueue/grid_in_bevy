use crate::world::chunk::{ChunkSize, ChunkTilePos};
use bevy::ecs::entity::{EntityMapper, MapEntities};
use bevy::ecs::query::QueryIter;
use bevy::ecs::reflect::ReflectMapEntities;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use std::ops::{Index, IndexMut};

#[derive(Component, Reflect, Debug)]
#[reflect(Component, MapEntities)]
pub struct TileStorage(Vec<Option<Entity>>);

impl TileStorage {
    pub fn new(chunk_size: ChunkSize) -> Self {
        Self(vec![None; chunk_size.count()])
    }
}

impl FromWorld for TileStorage {
    fn from_world(world: &mut World) -> Self {
        let chunk_size = world.get_resource::<ChunkSize>().unwrap();
        Self::new(*chunk_size)
    }
}

impl MapEntities for TileStorage {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        for tile in self.0.iter_mut().flatten() {
            *tile = entity_mapper.map_entity(*tile);
        }
    }
}

pub struct ChunkTiles<'t> {
    tiles: &'t mut TileStorage,
    chunk_size: ChunkSize,
}

impl<'t> ChunkTiles<'t> {
    pub fn new(tiles: &'t mut TileStorage, chunk_size: ChunkSize) -> Self {
        Self { tiles, chunk_size }
    }

    pub fn get(&self, pos: ChunkTilePos) -> Option<Entity> {
        pos.validate_bounds(self.chunk_size);
        self.tiles.0[pos.index(self.chunk_size)]
    }

    pub fn get_unchecked(&self, pos: ChunkTilePos) -> Option<Entity> {
        debug_assert!(pos.within_bounds(self.chunk_size));
        self.tiles.0[pos.index(self.chunk_size)]
    }

    pub fn set(&mut self, pos: ChunkTilePos, entity: Option<Entity>) {
        pos.validate_bounds(self.chunk_size);
        self.tiles.0[pos.index(self.chunk_size)] = entity;
    }

    pub fn set_unchecked(&mut self, pos: ChunkTilePos, entity: Option<Entity>) {
        debug_assert!(pos.within_bounds(self.chunk_size));
        self.tiles.0[pos.index(self.chunk_size)] = entity;
    }
}

impl<'t> Index<ChunkTilePos> for ChunkTiles<'t> {
    type Output = Option<Entity>;

    fn index(&self, index: ChunkTilePos) -> &Self::Output {
        index.validate_bounds(self.chunk_size);
        &self.tiles.0[index.index(self.chunk_size)]
    }
}

impl<'t> IndexMut<ChunkTilePos> for ChunkTiles<'t> {
    fn index_mut(&mut self, index: ChunkTilePos) -> &mut Self::Output {
        index.validate_bounds(self.chunk_size);
        &mut self.tiles.0[index.index(self.chunk_size)]
    }
}
