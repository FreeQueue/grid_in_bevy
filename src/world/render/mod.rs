use bevy::math::UVec2;
use bevy::prelude::{Component, Deref, Resource};

#[derive(Resource, Debug, Default, Copy, Clone)]
pub struct TilemapRenderSettings {
    pub render_chunk_size: UVec2,
}

#[derive(Component, Deref, Debug, Default, Clone)]
pub struct TilemapTileSize(UVec2);
