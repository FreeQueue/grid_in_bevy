use bevy::prelude::*;

#[derive(Component, Deref, DerefMut, Reflect, Eq, PartialEq, Default, Clone, Copy, Debug, Hash)]
#[reflect(Component)]
pub struct TilePos(IVec2);

impl TilePos {
    pub fn new(x: i32, y: i32) -> Self {
        Self(IVec2::new(x, y))
    }
}
