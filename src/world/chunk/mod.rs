use bevy::ecs::entity::{EntityMapper, MapEntities};
use bevy::ecs::reflect::ReflectMapEntities;

use bevy::prelude::*;

mod storage;
mod systems;

#[derive(Bundle, Reflect, Debug)]
pub struct ChunkBundle {}

#[derive(Component, Deref, DerefMut, Reflect, Eq, PartialEq, Default, Clone, Copy, Debug, Hash)]
pub struct ChunkPos(IVec2);

impl ChunkPos {
    pub fn new(x: i32, y: i32) -> Self {
        Self(IVec2::new(x, y))
    }
}

#[derive(Resource, Reflect, Deref, Clone, Copy, Debug)]
pub struct ChunkSize(UVec2);

impl ChunkSize {
    pub const CHUNK_SIZE: ChunkSize = ChunkSize::new(16, 16);
    pub const fn new(x: u32, y: u32) -> Self {
        Self(UVec2::new(x, y))
    }
    pub fn count(&self) -> usize {
        (self.x * self.y) as usize
    }
}

impl Default for ChunkSize {
    fn default() -> Self {
        Self::CHUNK_SIZE
    }
}

#[derive(Component, Reflect, Deref, Default, Clone, Copy, Debug, Hash)]
pub struct ChunkTilePos(UVec2);

impl ChunkTilePos {
    pub const fn new(x: u32, y: u32) -> Self {
        Self(UVec2::new(x, y))
    }

    pub fn index(&self, chunk_size: ChunkSize) -> usize {
        (self.y * chunk_size.x + self.x) as usize
    }

    pub fn within_bounds(&self, chunk_size: ChunkSize) -> bool {
        self.x < chunk_size.x && self.y < chunk_size.y
    }

    pub fn validate_bounds(&self, chunk_size: ChunkSize) {
        assert!(
            self.within_bounds(chunk_size),
            "{:?} out of bounds: {:?}",
            self,
            chunk_size
        );
    }
}
